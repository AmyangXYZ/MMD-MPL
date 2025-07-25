import { useCallback, useRef } from "react"

export default function CodeEditor({
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

  const lines = value.split("\n")
  const lineCount = Math.max(lines.length, 1)

  // Sync scroll between all elements
  const syncScroll = useCallback((source: "textarea" | "lineNumbers") => {
    if (!textareaRef.current || !lineNumbersRef.current || !syntaxHighlightRef.current) return

    if (source === "textarea") {
      const scrollTop = textareaRef.current.scrollTop
      lineNumbersRef.current.scrollTop = scrollTop
      syntaxHighlightRef.current.scrollTop = scrollTop
    } else if (source === "lineNumbers") {
      const scrollTop = lineNumbersRef.current.scrollTop
      textareaRef.current.scrollTop = scrollTop
      syntaxHighlightRef.current.scrollTop = scrollTop
    }
  }, [])

  // Handle scroll events
  const handleTextareaScroll = useCallback(() => {
    syncScroll("textarea")
  }, [syncScroll])

  const handleLineNumbersScroll = useCallback(() => {
    syncScroll("lineNumbers")
  }, [syncScroll])

  // Simple syntax highlighting function
  const highlightSyntax = (text: string) => {
    return text.split("\n").map((line, index) => {
      // Preserve original line including whitespace
      if (!line.trim()) {
        return (
          <div key={index} className="h-6">
            &nbsp;
          </div>
        )
      }

      // Split line into words while preserving whitespace
      const parts = line.split(/(\s+)/)

      return (
        <div key={index} className="h-6 leading-6">
          {parts.map((part, partIndex) => {
            // If it's whitespace, render as-is
            if (/^\s+$/.test(part)) {
              return <span key={partIndex}>{part}</span>
            }

            // Simple keyword highlighting
            if (part === "@pose") {
              return (
                <span key={partIndex} className="text-blue-600 dark:text-blue-300 font-bold">
                  {part}
                </span>
              )
            }
            if (part === "{" || part === "}" || part === ";") {
              return (
                <span key={partIndex} className="text-gray-500 dark:text-gray-500">
                  {part}
                </span>
              )
            }
            if (/^(bend|turn|sway)$/i.test(part)) {
              return (
                <span key={partIndex} className="text-green-700 dark:text-green-200 font-bold">
                  {part}
                </span>
              )
            }
            if (/^(forward|backward|left|right)$/i.test(part)) {
              return (
                <span key={partIndex} className="text-purple-700 dark:text-purple-200 font-bold">
                  {part}
                </span>
              )
            }
            if (/^\d+(?:\.\d+)?$/.test(part)) {
              return (
                <span key={partIndex} className="text-orange-700 dark:text-orange-200 font-bold">
                  {part}
                </span>
              )
            }

            // Default color for other text
            return (
              <span key={partIndex} className="text-gray-600 dark:text-gray-400">
                {part}
              </span>
            )
          })}
        </div>
      )
    })
  }

  return (
    <div className="flex flex-1 h-[calc(30vh-4rem)] md:h-[calc(100vh-4rem)] border border-gray-200 dark:border-gray-700 rounded-md overflow-hidden bg-white dark:bg-gray-900">
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
            WebkitTextFillColor: "inherit",
            wordWrap: "break-word",
            overflowWrap: "break-word",
            margin: 0,
            border: "none",
            lineHeight: "1.5rem",
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
            WebkitTextFillColor: "transparent",
            wordWrap: "break-word",
            overflowWrap: "break-word",
            margin: 0,
            border: "none",
            lineHeight: "1.5rem",
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
              wordWrap: "break-word",
              overflowWrap: "break-word",
              lineHeight: "1.5rem",
            }}
          >
            {placeholder}
          </div>
        )}
      </div>
    </div>
  )
}
