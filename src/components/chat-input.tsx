import { ArrowUp } from "lucide-react"
import { Button } from "./ui/button"
import { Textarea } from "./ui/textarea"
import { motion } from "framer-motion"
import { Card, CardDescription, CardHeader } from "./ui/card"

import { useState, useEffect, useRef, useCallback, Dispatch, SetStateAction } from "react"
import { Pose, MovableBones } from "@/lib/pose"
import { MPLInterpreter } from "@/lib/mpl"

const suggestedPoses: string[] = [
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
  "shoulder_l bend forward 30; arm_l bend forward 50; arm_l sway left 20",
  "pinky_r_0 bend forward 60",
  "center bend forward 20",
  "head bend forward 25; neck bend forward 20; upper_body bend forward 15",
  "arm_l bend forward 40; arm_r bend forward 40; head turn right 15",
] as const

export default function ChatInput({
  setPose,
  setSmoothUpdate,
}: {
  setPose: Dispatch<SetStateAction<Pose>>
  setSmoothUpdate: (smoothUpdate: boolean) => void
}) {
  const textareaRef = useRef<HTMLTextAreaElement>(null)
  const [displayedPoses, setDisplayedPoses] = useState<string[]>([])

  // Function to get 4 random poses
  const getRandomPoses = () => {
    const shuffled = [...suggestedPoses].sort(() => 0.5 - Math.random())
    return shuffled.slice(0, 4)
  }

  // Initialize with random poses on component mount
  useEffect(() => {
    setDisplayedPoses(getRandomPoses())
  }, [])

  const [description, setDescription] = useState("")

  const generatePose = useCallback(
    async (description: string) => {
      resetHeight()

      const poseData = MPLInterpreter(description)
      if (!poseData) {
        return
      }
      // setDescription("")
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
    if (textareaRef.current) {
      adjustHeight()
    }
  }, [])

  const adjustHeight = () => {
    if (textareaRef.current) {
      textareaRef.current.style.height = "auto"
      textareaRef.current.style.height = `${textareaRef.current.scrollHeight + 2}px`
    }
  }

  const resetHeight = () => {
    if (textareaRef.current) {
      textareaRef.current.style.height = "auto"
    }
  }
  return (
    <>
      <div className="relative w-full flex flex-col gap-3">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
          {displayedPoses.map((pose, i) => (
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: 20 }}
              transition={{ delay: 0.05 * i }}
              key={`suggested-pose-${pose}-${i}`}
              className={i > 1 ? "hidden sm:block" : "block"}
            >
              <Card
                key={i}
                className={`bg-white/50 hover:bg-pink-100/70 py-0 gap-0 h-full w-full cursor-pointer backdrop-blur-[3px] shadow-lg ${
                  i >= 2 ? "hidden md:block" : ""
                }`}
                onClick={() => {
                  generatePose(pose)
                }}
              >
                <CardHeader className="py-2 gap-0">
                  <CardDescription className="py-1 text-zinc-800 ">{pose}</CardDescription>
                </CardHeader>
              </Card>
            </motion.div>
          ))}
        </div>

        <div className="relative w-full">
          <Textarea
            ref={textareaRef}
            className="max-h-[calc(75dvh)] overflow-hidden resize-none rounded-2xl !text-base bg-white/50 text-zinc-800 pb-4 backdrop-blur-[3px] shadow-lg px-4"
            value={description}
            onChange={(e) => {
              setDescription(e.target.value)
              adjustHeight()
            }}
            onKeyDown={(e) => {
              if (e.key === "Enter" && description.trim().length > 0) {
                e.preventDefault()
                generatePose(description)
              }
            }}
            disabled={false}
            placeholder={"<bone> <action> <direction> <degree>"}
          />
        </div>

        <div className="absolute bottom-0 right-0 p-2 w-fit flex flex-row justify-end">
          <Button
            size="icon"
            className="rounded-full h-fit w-fit p-1"
            disabled={description.length === 0}
            onClick={() => generatePose(description)}
          >
            <ArrowUp className="size-5" />
          </Button>
        </div>
      </div>
    </>
  )
}
