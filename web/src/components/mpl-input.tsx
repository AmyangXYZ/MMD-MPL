import { MPLToPose, PoseToMPL, Pose } from "@/lib/mpl"
import { SetStateAction, Dispatch, useCallback, useState, useEffect, useRef } from "react"
import { Button } from "./ui/button"
import { Import, RefreshCw } from "lucide-react"
import Link from "next/link"
import Image from "next/image"
import CodeEditor from "./code-editor"
import { useMPLCompiler } from "../../hooks/useMPLCompiler"
import { MPLBoneState } from "mmd-mpl"

export default function MPLInput({
  applyBoneStates,
  loadVpd,
  modelLoaded,
}: {
  applyBoneStates: (boneStates: MPLBoneState[]) => void
  loadVpd: (url: string) => Promise<Pose | null>
  modelLoaded: boolean
}) {
  const mplCompiler = useMPLCompiler()

  const [statement, setStatement] = useState(`@pose a {
        upper_body bend forward 11.611;
    upper_body sway left 8.889;
shoulder_r bend backward 13.441;
shoulder_r sway left 3.702;
ankle_r bend forward 60.000;
ankle_r turn left 4.310;
ankle_r sway left 4.713;
knee_l bend backward 33.750;
ankle_l bend forward 43.101;
ankle_l turn right 1.853;
ankle_l sway right 1.486;
upper_body2 bend backward 9.333;
upper_body2 turn left 1.167;
upper_body2 sway right 14.000;
neck bend forward 8.525;
neck turn right 6.611;
neck sway right 12.633;
arm_twist_r turn right 5.063;
elbow_r bend forward 135.000;
wrist_twist_r turn right 27.000;
wrist_r bend backward 30.000;
thumb_0_r bend backward 3.018;
thumb_0_r sway left 6.398;
pinky_0_r bend backward 15.000;
pinky_0_r sway right 2.637;
pinky_1_r bend forward 18.000;
ring_0_r bend backward 13.037;
ring_0_r sway right 0.804;
ring_1_r bend forward 18.406;
middle_1_r bend forward 22.500;
index_0_r bend forward 17.231;
index_0_r sway right 2.843;
index_1_r bend forward 21.375;
shoulder_l bend backward 14.003;
shoulder_l sway left 2.156;
arm_l bend forward 6.750;
arm_twist_l turn left 18.000;
elbow_l bend forward 135.000;
wrist_twist_l turn left 16.198;
wrist_l sway left 11.607;
thumb_2_l bend forward 20.004;
pinky_1_l bend forward 31.513;
ring_1_l bend forward 31.500;
middle_1_l bend forward 42.000;
index_1_l bend forward 54.000;
leg_r bend forward 18.667;
leg_r turn right 8.139;
leg_r sway left 0.963;
leg_l bend forward 32.118;
leg_l turn left 4.180;
leg_l sway left 0.819;
  }`)

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

  const resetPose = useCallback(() => {
    applyBoneStates([])
  }, [applyBoneStates])

  useEffect(() => {
    if (modelLoaded && mplCompiler) {
      try {
        const boneStates = mplCompiler.compile(statement)
        console.log(boneStates)
        applyBoneStates(boneStates)
      } catch (error) {
        console.error(error)
      }
    }
  }, [statement, modelLoaded, mplCompiler, applyBoneStates])

  return (
    <div className="flex flex-col gap-1 w-full h-full">
      <div className="flex flex-row gap-2 px-6 pt-2 z-100 items-center justify-between">
        <h3 className="scroll-m-20 text-xl font-semibold tracking-tight hidden md:block">
          MMD Pose Language (MPL) Editor
        </h3>
        <h3 className="scroll-m-20 text-lg font-semibold tracking-tight md:hidden">MPL Editor</h3>
        <div className="flex flex-row gap-2">
          <div className="relative hidden md:block">
            <input
              type="file"
              accept=".vpd"
              onChange={handleFileUpload}
              className="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
              id="pose-upload"
            />
            <Button
              onClick={() => {
                setStatement("")
              }}
              className="flex"
              size="sm"
            >
              <Import className="size-4" />
              <span className="text-xs">Import VPD</span>
            </Button>
          </div>

          <Button
            onClick={() => {
              setStatement("")
              resetPose()
            }}
            className="flex gap-2 bg-black text-white"
            size="sm"
            variant="outline"
          >
            <RefreshCw className="size-4" />
          </Button>
          <Button size="sm" asChild className="bg-black text-white px-2.5" variant="outline">
            <Link href="https://github.com/AmyangXYZ/MPL" target="_blank" className="flex gap-2">
              <Image src="/github-mark-white.svg" alt="GitHub" width={18} height={18} />
            </Link>
          </Button>
        </div>
      </div>

      <div className="flex-1 py-2 px-6">
        <CodeEditor value={statement} onChange={setStatement} placeholder="head turn left 30;" />
      </div>
    </div>
  )
}
