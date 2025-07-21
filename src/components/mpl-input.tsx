import { MPLToPose, PoseToMPL, BONES, ACTIONS, DIRECTIONS, BONE_ACTION_RULES } from "@/lib/mpl"
import { Pose } from "@/lib/pose"
import { SetStateAction, Dispatch, useCallback, useState, useEffect, useRef } from "react"
import { Button } from "./ui/button"
import { Import, RefreshCw } from "lucide-react"
import Link from "next/link"
import Image from "next/image"

// Code Editor Component with line numbers and limited height
function CodeEditor({
  value,
  onChange,
  placeholder,
}: {
  value: string
  onChange: (value: string) => void
  placeholder?: string
}) {
  const textareaRef = useRef<HTMLTextAreaElement>(null)
  const lineNumbersRef = useRef<HTMLDivElement>(null)
  const syntaxHighlightRef = useRef<HTMLDivElement>(null)

  const lines = value.split('\n')
  const lineCount = Math.max(lines.length, 1)

  // Sync scroll between all elements
  const syncScroll = useCallback((source: 'textarea' | 'lineNumbers') => {
    if (!textareaRef.current || !lineNumbersRef.current || !syntaxHighlightRef.current) return

    if (source === 'textarea') {
      const scrollTop = textareaRef.current.scrollTop
      lineNumbersRef.current.scrollTop = scrollTop
      syntaxHighlightRef.current.scrollTop = scrollTop
    } else if (source === 'lineNumbers') {
      const scrollTop = lineNumbersRef.current.scrollTop
      textareaRef.current.scrollTop = scrollTop
      syntaxHighlightRef.current.scrollTop = scrollTop
    }
  }, [])

  // Handle scroll events
  const handleTextareaScroll = useCallback(() => {
    syncScroll('textarea')
  }, [syncScroll])

  const handleLineNumbersScroll = useCallback(() => {
    syncScroll('lineNumbers')
  }, [syncScroll])

  // Syntax highlighting function for MPL with real-time validation
  const highlightSyntax = (text: string) => {
    return text.split('\n').map((line, index) => {
      const trimmedLine = line.trim()
      if (!trimmedLine) return <div key={index} className="h-6">&nbsp;</div>

      // Split line into parts
      const parts = trimmedLine.split(' ')

      const renderPart = (part: string, partIndex: number, expectedType: 'bone' | 'action' | 'direction' | 'degrees') => {
        let isValid = false
        let colorClass = "text-red-500 dark:text-red-400" // Default to error color

        if (expectedType === 'bone') {
          isValid = BONES.hasOwnProperty(part.toLowerCase())
          colorClass = isValid
            ? "text-blue-700 dark:text-blue-200 font-bold"
            : "text-red-600 dark:text-red-400 font-medium"
        } else if (expectedType === 'action') {
          isValid = ACTIONS.includes(part.toLowerCase())
          colorClass = isValid
            ? "text-green-700 dark:text-green-200 font-bold"
            : "text-red-600 dark:text-red-400 font-medium"
        } else if (expectedType === 'direction') {
          isValid = DIRECTIONS.includes(part.toLowerCase())
          // Additional validation: check if this bone-action-direction combo is valid
          if (isValid && parts.length >= 3) {
            const bone = parts[0].toLowerCase()
            const action = parts[1].toLowerCase()
            const direction = part.toLowerCase()
            const boneRules = BONE_ACTION_RULES[bone]
            const actionRules = boneRules?.[action]
            const isComboValid = actionRules?.[direction] !== undefined
            isValid = isComboValid
          }
          colorClass = isValid
            ? "text-purple-700 dark:text-purple-200 font-bold"
            : "text-red-600 dark:text-red-400 font-medium"
        } else if (expectedType === 'degrees') {
          const degrees = parseFloat(part)
          isValid = !isNaN(degrees) && degrees >= 0
          // Additional validation: check if degrees are within the limit
          if (isValid && parts.length >= 4) {
            const bone = parts[0].toLowerCase()
            const action = parts[1].toLowerCase()
            const direction = parts[2].toLowerCase()
            const boneRules = BONE_ACTION_RULES[bone]
            const actionRules = boneRules?.[action]
            const rule = actionRules?.[direction]
            if (rule && degrees > rule.limit) {
              isValid = false
            }
          }
          colorClass = isValid
            ? "text-orange-700 dark:text-orange-200 font-bold"
            : "text-red-600 dark:text-red-400 font-medium"
        }

        return <span key={partIndex} className={colorClass}>{part}</span>
      }

      return (
        <div key={index} className="h-6 leading-6">
          {parts.map((part, partIndex) => {
            let expectedType: 'bone' | 'action' | 'direction' | 'degrees' | null = null

            if (partIndex === 0) expectedType = 'bone'
            else if (partIndex === 1) expectedType = 'action'
            else if (partIndex === 2) expectedType = 'direction'
            else if (partIndex === 3) expectedType = 'degrees'

            const element = expectedType
              ? renderPart(part, partIndex, expectedType)
              : <span key={partIndex} className="text-red-500 dark:text-red-400">{part}</span>

            return (
              <span key={partIndex}>
                {element}
                {partIndex < parts.length - 1 && (
                  <span className="text-gray-600 dark:text-gray-400"> </span>
                )}
              </span>
            )
          })}
        </div>
      )
    })
  }

  return (
    <div className="flex flex-1 h-[calc(100vh-6rem)] border border-gray-200 dark:border-gray-700 rounded-md overflow-hidden bg-white dark:bg-gray-900">
      {/* Line Numbers */}
      <div
        ref={lineNumbersRef}
        className="bg-gray-50 dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 overflow-y-auto overflow-x-hidden [&::-webkit-scrollbar]:hidden [-ms-overflow-style:none] [scrollbar-width:none]"
        style={{
          width: `${Math.max(2.5, lineCount.toString().length * 0.8 + 1)}rem`,
        }}
        onScroll={handleLineNumbersScroll}
      >
        <div className="px-2 py-2 text-xs text-gray-500 dark:text-gray-400 font-mono leading-6 select-none">
          {Array.from({ length: lineCount }, (_, i) => (
            <div key={i + 1} className="text-right h-6 leading-6">
              {i + 1}
            </div>
          ))}
        </div>
      </div>

      {/* Code Area */}
      <div className="flex-1 relative">
        {/* Syntax Highlighted Background */}
        <div
          ref={syntaxHighlightRef}
          className="absolute inset-0 w-full h-full p-2 bg-transparent resize-none outline-none font-mono text-sm leading-6 overflow-y-auto overflow-x-hidden pointer-events-none scrollbar-none whitespace-pre-wrap"
          style={{
            tabSize: 2,
            WebkitTextFillColor: 'inherit',
            wordWrap: 'break-word',
            overflowWrap: 'break-word',
            margin: 0,
            border: 'none',
            lineHeight: '1.5rem',
          }}
        >
          {value ? highlightSyntax(value) : null}
        </div>

        {/* Actual Textarea */}
        <textarea
          ref={textareaRef}
          className="absolute inset-0 w-full h-full p-2 bg-transparent text-transparent caret-gray-900 dark:caret-white resize-none outline-none font-mono text-sm leading-6 overflow-y-auto overflow-x-hidden scrollbar-thin scrollbar-thumb-gray-300 dark:scrollbar-thumb-gray-600 scrollbar-track-transparent selection:bg-blue-200/50 dark:selection:bg-blue-800/50 selection:bg-opacity-50 dark:selection:bg-opacity-50 whitespace-pre-wrap"
          style={{
            tabSize: 2,
            WebkitTextFillColor: 'transparent',
            wordWrap: 'break-word',
            overflowWrap: 'break-word',
            margin: 0,
            border: 'none',
            lineHeight: '1.5rem',
          }}
          value={value}
          onChange={(e) => onChange(e.target.value)}
          onScroll={handleTextareaScroll}
          placeholder={placeholder}
          autoFocus
          spellCheck={false}
        />

        {/* Placeholder */}
        {!value && (
          <div
            className="absolute top-2 left-2 text-gray-400 dark:text-gray-500 font-mono text-sm pointer-events-none leading-6 whitespace-pre-wrap select-none"
            style={{
              tabSize: 2,
              wordWrap: 'break-word',
              overflowWrap: 'break-word',
              lineHeight: '1.5rem',
            }}
          >
            {placeholder}
          </div>
        )}
      </div>
    </div>
  )
}

export default function MPLInput({
  setPose,
  resetPose,
  loadVpd,
}: {
  setPose: Dispatch<SetStateAction<Pose>>
  resetPose: () => void
  loadVpd: (url: string) => Promise<Pose | null>
}) {
  const [statement, setStatement] = useState(``)



  const handleFileUpload = useCallback(
    async (event: React.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0]
      if (!file) return

      if (file.name.endsWith(".vpd")) {
        const url = URL.createObjectURL(file)
        const pose = await loadVpd(url)
        if (pose) {
          setStatement(PoseToMPL(pose).replaceAll(";", ";\n"))
        }
      }

      event.target.value = ""
    },
    [setStatement, loadVpd]
  )

  const generatePose = useCallback(
    async (statement: string) => {
      if (statement === "") {
        resetPose()
        return
      }

      const poseData = MPLToPose(statement)

      if (poseData) {
        setPose(poseData)
      }
    },
    [setPose, resetPose]
  )

  useEffect(() => {
    generatePose(statement)
  }, [statement, generatePose])

  return (
    <div className="flex flex-col gap-2 w-full h-full">
      <div className="flex flex-row gap-2 px-6 pt-2 z-100 items-center justify-between">
        <h3 className="scroll-m-20 text-xl font-semibold tracking-tight">
          MMD Pose Language (MPL) Editor
        </h3>
        <div className="flex flex-row gap-2">
          <div className="relative mr-1">
            <input
              type="file"
              accept=".vpd"
              onChange={handleFileUpload}
              className="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
              id="pose-upload"
            />
            <Button onClick={() => { setStatement(""); }} className="flex" size="sm">
              <Import className="size-4" />
              <span className="text-xs">Import VPD</span>
            </Button>
          </div>

          <Button onClick={() => { setStatement(""); resetPose() }} className="flex gap-2" size="sm">
            <RefreshCw className="size-4" />
            <span className="text-xs">Reset</span>
          </Button>
          <Button className="flex gap-2" size="sm">
            <Link href="https://github.com/AmyangXYZ/MPL" target="_blank">
              <Image src="/github-mark-white.svg" alt="GitHub" width={18} height={18} />
            </Link>
            <span className="text-xs">Github</span>
          </Button>
        </div>
      </div>

      <div className="flex-1 py-2 px-6">
        <CodeEditor
          value={statement}
          onChange={setStatement}
          placeholder="head turn left 30;"
        />
      </div>
    </div>
  )
}
