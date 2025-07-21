import { VALID_STATEMENTS, MPLToPose } from "@/lib/mpl"
import { MovableBones, Pose } from "@/lib/pose"
import { SetStateAction, Dispatch, useCallback, useState, useEffect } from "react"
import { CommandGroup, CommandInput, CommandItem, CommandList, Command } from "./ui/command"
import { CardDescription, CardHeader } from "./ui/card"
import { motion } from "framer-motion"
import { Card } from "./ui/card"

const suggestedStatements: string[] = [
  "index_r_0 bend forward 90",
  "arm_l bend forward 40",
  "head turn right 30; head bend backward 20",
  "knee_l bend backward 20; leg_r bend forward 30; leg_l turn left 30",
  "thumb_l_0 bend forward 60",
  "neck bend forward 25",
  "arm_r bend forward 45; arm_r sway right 20; wrist_r bend forward 30",
  "head turn left 20; neck turn left 15; upper_body turn left 10",
  "middle_l_0 bend forward 75",
  "waist bend forward 30",
  "leg_l bend forward 45; leg_l sway left 25; ankle_l bend forward 15",
  "pinky_r_0 bend forward 60",
  "center bend forward 20",
  "head bend forward 25; neck bend forward 20; upper_body bend forward 15",
  "arm_l bend forward 40; arm_r bend forward 40; head turn right 15",
] as const

export default function MPLInput({
  setPose,
  setSmoothUpdate,
}: {
  setPose: Dispatch<SetStateAction<Pose>>
  setSmoothUpdate: (smoothUpdate: boolean) => void
}) {
  const [statement, setStatement] = useState("")
  const [showAvailableStatements, setShowAvailableStatements] = useState(false)

  const [displayedStatements, setDisplayedStatements] = useState<string[]>([])

  // Function to get 4 random poses
  const getRandomStatements = () => {
    const shuffled = [...suggestedStatements].sort(() => 0.5 - Math.random())
    return shuffled.slice(0, 4)
  }

  // Initialize with random poses on component mount
  useEffect(() => {
    setDisplayedStatements(getRandomStatements())
  }, [])

  const generatePose = useCallback(
    async (description: string) => {
      const poseData = MPLToPose(description)
      if (!poseData) {
        return
      }
      setSmoothUpdate(true)

      setPose((prev) => ({
        ...prev,
        description: poseData.description || prev.description,
        face: { ...prev.face, ...poseData.face },
        movableBones: { ...prev.movableBones, ...poseData.movableBones } as MovableBones,
        rotatableBones: { ...prev.rotatableBones, ...poseData.rotatableBones },
      }))
    },
    [setPose, setSmoothUpdate]
  )

  useEffect(() => {
    generatePose(statement)
  }, [statement, generatePose])

  return (
    <div className="flex flex-col gap-2">
      <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
        {displayedStatements.map((statement, i) => (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: 20 }}
            transition={{ delay: 0.05 * i }}
            key={`suggested-statement-${statement}-${i}`}
            className={i > 1 ? "hidden sm:block" : "block"}
          >
            <Card
              key={i}
              className={`bg-white/50 hover:bg-pink-100/70 py-0 gap-0 h-full w-full cursor-pointer backdrop-blur-[3px] shadow-lg ${i >= 2 ? "hidden md:block" : ""
                }`}
              onClick={() => {
                generatePose(statement)
              }}
            >
              <CardHeader className="py-2 gap-0">
                <CardDescription className="py-1 text-zinc-800 ">{statement}</CardDescription>
              </CardHeader>
            </Card>
          </motion.div>
        ))}
      </div>

      <Command className="bg-white/70 backdrop-blur-sm shadow-lg">
        {(statement || showAvailableStatements) && (
          <CommandList>
            <CommandGroup>
              {VALID_STATEMENTS.map((stmt: string, index: number) => (
                <CommandItem
                  key={index}
                  value={stmt}
                  onSelect={(currentValue) => {
                    setStatement(currentValue)
                  }}
                >
                  {stmt}
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        )}
        <CommandInput
          placeholder="MMD Pose Language - <bone> <action> <direction> <degrees>"
          className="h-20"
          value={statement}
          onValueChange={(value) => setStatement(value)}
          onFocus={() => {
            setShowAvailableStatements(true)
          }}
          onBlur={() => {
            setTimeout(() => {
              setShowAvailableStatements(false)
            }, 100)
          }}
        />
      </Command>
    </div>
  )
}
