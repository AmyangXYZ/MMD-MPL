import { useCallback, useState, useEffect } from "react"
import { Button } from "./ui/button"
import { Download, RefreshCw, Upload } from "lucide-react"
import Link from "next/link"
import Image from "next/image"
import { useMPLCompiler } from "@/hooks/useMPLCompiler"
import { MPLBoneFrame } from "mmd-mpl"
import CodeEditor from "./code-editor"

export default function MPLInput({
  loadVPD,
  modelLoaded,
  loadVMD,
}: {
  loadVPD: (url: string) => Promise<MPLBoneFrame[] | null>
  loadVMD: (url: string) => void
  modelLoaded: boolean
}) {
  const mplCompiler = useMPLCompiler()
  const [vmdUrl, setVmdUrl] = useState<string | null>(null)

  const [statement, setStatement] = useState(`@pose pose_0 {
    center bend forward 0;
    center turn left 0;
    center sway left 0;
    upper_body bend forward 0;
    upper_body turn left 0;
    upper_body sway right 0;
    upper_body2 bend backward 0;
    upper_body2 turn right 0;
    upper_body2 sway left 0;
    lower_body bend backward 0;
    lower_body turn right 0;
    lower_body sway left 0;
    neck bend backward 0;
    neck turn right 0;
    neck sway left 0;
    head bend forward 0;
    head turn left 0;
    head sway right 0;
    shoulder_l bend forward 0;
    shoulder_l turn right 0;
    shoulder_l sway left 0;
    shoulder_r bend backward 0;
    shoulder_r turn left 0;
    shoulder_r sway right 0;
    arm_l bend forward 40;
    arm_l sway right 5;
    arm_r bend forward 40;
    arm_r turn left 0;
    arm_r sway right 5;
    elbow_l bend forward 15;
    elbow_r bend forward 15;
    wrist_l bend forward 5;
    wrist_l turn right 15;
    wrist_l sway left 5;
    wrist_r bend forward 5;
    wrist_r turn left 15;
    wrist_r sway left 5;
    leg_l bend forward 0;
    leg_l turn left 0;
    leg_l sway right 0;
    leg_r bend forward 0;
    leg_r turn right 0;
    leg_r sway right 0;
    thumb_1_l bend forward 10;
    thumb_2_l bend forward 20;
    index_0_l bend forward 10;
    index_0_l sway left 5;
    index_1_l bend forward 30;
    index_2_l bend forward 25;
    middle_0_l bend forward 10;
    middle_0_l sway right 5;
    middle_1_l bend forward 40;
    middle_2_l bend forward 25;
    ring_0_l bend forward 15;
    ring_0_l sway right 0;
    ring_1_l bend forward 40;
    ring_2_l bend forward 25;
    pinky_0_l bend forward 15;
    pinky_0_l sway right 5;
    pinky_1_l bend forward 30;
    pinky_2_l bend forward 40;
    thumb_1_r bend forward 10;
    thumb_2_r bend forward 20;
    index_0_r bend forward 10;
    index_0_r sway right 5;
    index_1_r bend forward 30;
    index_2_r bend forward 25;
    middle_0_r bend forward 10;
    middle_0_r sway left 5;
    middle_1_r bend forward 40;
    middle_2_r bend forward 25;
    ring_0_r bend forward 15;
    ring_0_r sway right 0;
    ring_1_r bend forward 40;
    ring_2_r bend forward 25;
    pinky_0_r bend forward 15;
    pinky_0_r sway left 5;
    pinky_1_r bend forward 30;
    pinky_2_r bend forward 40;
}

@pose pose_1 {

}

@pose pose_2 {
    thumb_1_l bend forward 10;
    thumb_2_l bend forward 20;
    index_0_l bend forward 20;
    index_0_l sway left 5;
    index_1_l bend forward 75;
    index_2_l bend forward 25;
    middle_0_l bend forward 30;
    middle_0_l sway right 5;
    middle_1_l bend forward 75;
    middle_2_l bend forward 30;
    ring_0_l bend forward 40;
    ring_0_l sway right 0;
    ring_1_l bend forward 75;
    ring_2_l bend forward 20;
    pinky_0_l bend forward 45;
    pinky_0_l sway right 0;
    pinky_1_l bend forward 65;
    pinky_2_l bend forward 30;
}

@pose pose_3 {
    elbow_r bend forward 100;
}

@pose pose_4 {
    wrist_l bend forward 20;
    wrist_l turn right 40;
    wrist_l sway right 5;
}

@pose pose_5 {

}

@pose pose_6 {
    center bend forward 0;
    center turn left 0;
    center sway left 0;
    center move left 0;
    center move down 0;
    center move forward 0;
    upper_body bend backward 5;
    upper_body turn right 0;
    upper_body sway left 0;
    wrist_l bend backward 15;
    wrist_l turn left 25;
    wrist_l sway left 10;
}

@pose pose_7 {
    thumb_1_l bend forward 10;
    thumb_2_l bend forward 10;
    index_0_l bend forward 10;
    index_1_l bend forward 25;
    index_2_l bend backward 0;
    middle_0_l bend forward 5;
    middle_0_l sway left 0;
    middle_1_l bend forward 25;
    middle_2_l bend forward 0;
    ring_0_l bend forward 5;
    ring_0_l sway left 0;
    ring_1_l bend forward 20;
    ring_2_l bend backward 0;
    pinky_0_l bend forward 5;
    pinky_0_l sway left 0;
    pinky_1_l bend forward 5;
    pinky_2_l bend forward 0;
}

@pose pose_8 {
    shoulder_r bend backward 5;
    shoulder_r turn left 15;
    shoulder_r sway left 15;
    arm_r bend forward 5;
    arm_r turn left 45;
    arm_r sway right 0;
    wrist_r bend forward 5;
    wrist_r turn left 35;
    wrist_r sway right 5;
}

@pose pose_9 {
    neck bend forward 25;
    neck turn right 0;
    neck sway right 5;
    head bend forward 10;
    head turn right 0;
    head sway right 0;
}

@pose pose_10 {
    lower_body sway left 0;
    shoulder_l bend backward 30;
    shoulder_l turn left 10;
    shoulder_l sway left 5;
    arm_l bend backward 15;
    arm_l turn left 80;
    arm_l sway left 60;
    elbow_l bend forward 130;
    wrist_l bend backward 15;
    wrist_l turn right 5;
    wrist_l sway left 25;
    thumb_1_l bend forward 15;
    thumb_2_l bend forward 15;
    index_0_l bend forward 15;
    index_0_l sway left 5;
    index_1_l bend forward 35;
    index_2_l bend forward 30;
    middle_0_l bend forward 15;
    middle_0_l sway right 5;
    middle_1_l bend forward 45;
    middle_2_l bend forward 30;
    ring_0_l bend forward 20;
    ring_0_l sway right 5;
    ring_1_l bend forward 45;
    ring_2_l bend forward 25;
    pinky_0_l bend forward 15;
    pinky_0_l sway right 5;
    pinky_1_l bend forward 35;
    pinky_2_l bend forward 40;
    thumb_1_r bend forward 15;
    thumb_2_r bend forward 15;
    index_0_r bend forward 30;
    index_0_r sway right 5;
    index_1_r bend forward 90;
    index_2_r bend forward 20;
    middle_0_r bend forward 55;
    middle_0_r sway right 0;
    middle_1_r bend forward 90;
    middle_2_r bend forward 35;
    ring_0_r bend forward 70;
    ring_0_r sway left 5;
    ring_1_r bend forward 90;
    ring_2_r bend forward 15;
    pinky_0_r bend forward 80;
    pinky_0_r sway right 5;
    pinky_1_r bend forward 90;
    pinky_2_r bend forward 15;
}

@pose pose_11 {
    center bend forward 0;
    center turn left 0;
    center sway left 0;
    center move left 0;
    center move down 0;
    center move forward 0;
}

@pose pose_12 {
    shoulder_l bend backward 25;
    shoulder_l turn left 5;
    shoulder_l sway left 5;
    arm_l bend backward 15;
    arm_l turn left 85;
    arm_l sway left 65;
    wrist_l bend backward 5;
    wrist_l turn left 10;
    wrist_l sway left 25;
}

@pose pose_13 {
    shoulder_l bend backward 30;
    shoulder_l turn left 10;
    shoulder_l sway left 5;
    arm_l bend backward 15;
    arm_l turn left 90;
    arm_l sway left 55;
    elbow_l bend forward 130;
    wrist_l bend backward 5;
    wrist_l turn left 5;
    wrist_l sway left 35;
}

@pose pose_14 {
    upper_body bend forward 0;
    upper_body turn left 0;
    upper_body sway right 0;
    upper_body2 bend backward 0;
    upper_body2 turn right 0;
    upper_body2 sway left 0;
    neck bend forward 20;
    neck turn right 5;
    neck sway left 5;
    head bend forward 5;
    head turn right 0;
    head sway right 5;
    shoulder_r bend backward 5;
    shoulder_r turn left 15;
    shoulder_r sway left 15;
    arm_r bend forward 5;
    arm_r turn left 45;
    arm_r sway right 0;
    elbow_r bend forward 100;
    wrist_r bend forward 5;
    wrist_r turn left 35;
    wrist_r sway right 5;
    leg_l bend forward 0;
    leg_l turn left 0;
    leg_l sway right 0;
    leg_r bend forward 0;
    leg_r turn right 0;
    leg_r sway right 0;
}

@pose pose_15 {
    center bend forward 0;
    center turn left 0;
    center sway left 0;
}

@pose pose_16 {
    shoulder_l bend backward 30;
    shoulder_l turn left 10;
    shoulder_l sway left 5;
    arm_l bend backward 5;
    arm_l turn left 85;
    arm_l sway left 55;
    elbow_l bend forward 130;
    wrist_l bend backward 15;
    wrist_l turn left 15;
    wrist_l sway left 25;
}

@pose pose_17 {
    center bend forward 0;
    center turn left 0;
    center sway left 0;
    center move left 0;
    center move down 0;
    center move forward 0;
    upper_body bend backward 5;
    upper_body turn left 0;
    upper_body sway right 0;
    lower_body bend backward 0;
    lower_body turn right 0;
    lower_body sway left 0;
    neck bend forward 20;
    neck turn right 0;
    neck sway left 5;
    head bend forward 5;
    head turn left 10;
    head sway right 5;
    shoulder_l bend backward 25;
    shoulder_l turn left 5;
    shoulder_l sway left 5;
    arm_l bend backward 15;
    arm_l turn left 90;
    arm_l sway left 55;
    elbow_l bend forward 135;
    wrist_l bend backward 5;
    wrist_l turn left 10;
    wrist_l sway left 15;
}

@pose pose_18 {
    center bend forward 0;
    center turn left 0;
    center sway left 0;
    center move left 0;
    center move down 0;
    center move forward 0;
}

@pose pose_19 {
    neck bend forward 5;
    neck turn left 0;
    neck sway left 0;
    head bend forward 0;
    head turn left 0;
    head sway right 0;
    shoulder_l bend backward 30;
    shoulder_l turn left 10;
    shoulder_l sway left 5;
    arm_l bend backward 15;
    arm_l turn left 90;
    arm_l sway left 60;
    elbow_l bend forward 130;
    wrist_l bend backward 25;
    wrist_l turn left 5;
    wrist_l sway left 35;
    thumb_1_l bend forward 10;
    thumb_2_l bend forward 20;
    index_0_l bend forward 10;
    index_0_l sway left 5;
    index_1_l bend forward 30;
    index_2_l bend forward 25;
    middle_0_l bend forward 10;
    middle_0_l sway right 5;
    middle_1_l bend forward 40;
    middle_2_l bend forward 25;
    ring_0_l bend forward 15;
    ring_0_l sway right 0;
    ring_1_l bend forward 40;
    ring_2_l bend forward 25;
    pinky_0_l bend forward 15;
    pinky_0_l sway right 5;
    pinky_1_l bend forward 30;
    pinky_2_l bend forward 40;
}

@pose pose_20 {
    elbow_l bend forward 130;
}

@pose pose_21 {
    center bend forward 0;
    center turn left 0;
    center sway left 0;
    center move left 0;
    center move forward 0;
    upper_body bend forward 0;
    upper_body turn left 0;
    upper_body sway right 0;
    upper_body2 bend backward 0;
    upper_body2 turn right 0;
    upper_body2 sway left 0;
    lower_body bend backward 0;
    lower_body turn right 0;
    lower_body sway left 0;
    leg_l bend forward 0;
    leg_l turn left 0;
    leg_l sway right 0;
    leg_r bend forward 0;
    leg_r turn right 0;
    leg_r sway right 0;
    thumb_1_l bend forward 10;
    thumb_2_l bend forward 10;
    index_0_l bend forward 10;
    index_1_l bend forward 25;
    index_2_l bend backward 0;
    middle_0_l bend forward 5;
    middle_0_l sway left 0;
    middle_1_l bend forward 25;
    middle_2_l bend forward 0;
    ring_0_l bend forward 5;
    ring_0_l sway left 0;
    ring_1_l bend forward 20;
    ring_2_l bend backward 0;
    pinky_0_l bend forward 5;
    pinky_0_l sway left 0;
    pinky_1_l bend forward 5;
    pinky_2_l bend forward 0;
}

@pose pose_22 {
    wrist_l bend backward 30;
    wrist_l turn left 5;
    wrist_l sway left 40;
}

@pose pose_23 {
    neck bend backward 0;
    neck turn right 0;
    neck sway right 0;
    head bend forward 5;
}

@pose pose_24 {
    shoulder_l bend backward 5;
    arm_l bend forward 45;
    arm_l turn right 0;
    arm_l sway right 10;
}

@pose pose_25 {
    elbow_l bend forward 10;
    wrist_l bend forward 10;
    wrist_l turn left 0;
    wrist_l sway right 10;
}

@pose pose_26 {
    center bend forward 0;
    center turn left 0;
    center sway left 0;
}

@pose pose_27 {
    wrist_l bend forward 5;
    wrist_l turn left 0;
    wrist_l sway left 10;
}

@pose pose_28 {
    center bend forward 0;
    center turn left 0;
    center sway left 0;
    center move right 0;
    center move forward 0;
    upper_body bend backward 5;
    upper_body sway right 0;
    lower_body sway left 0;
    neck bend forward 10;
    neck turn left 0;
    neck sway right 5;
    head bend forward 5;
    head turn left 0;
    head sway left 0;
    shoulder_l bend backward 5;
    shoulder_r bend backward 5;
    shoulder_r turn left 15;
    shoulder_r sway left 15;
    arm_l bend forward 45;
    arm_l turn left 0;
    arm_l sway right 5;
    arm_r bend forward 5;
    arm_r turn left 45;
    arm_r sway right 0;
    elbow_l bend forward 10;
    wrist_l bend forward 5;
    wrist_l turn right 15;
    wrist_l sway left 5;
}

@pose pose_29 {
    wrist_r bend forward 5;
    wrist_r turn left 35;
    wrist_r sway right 5;
    thumb_1_l bend forward 10;
    thumb_2_l bend forward 20;
    index_0_l bend forward 10;
    index_0_l sway left 5;
    index_1_l bend forward 30;
    index_2_l bend forward 25;
    middle_0_l bend forward 10;
    middle_0_l sway right 5;
    middle_1_l bend forward 40;
    middle_2_l bend forward 25;
    ring_0_l bend forward 15;
    ring_0_l sway right 0;
    ring_1_l bend forward 40;
    ring_2_l bend forward 25;
    pinky_0_l bend forward 15;
    pinky_0_l sway right 5;
    pinky_1_l bend forward 30;
    pinky_2_l bend forward 40;
    thumb_1_r bend forward 15;
    thumb_2_r bend forward 15;
    index_0_r bend forward 30;
    index_0_r sway right 5;
    index_1_r bend forward 90;
    index_2_r bend forward 20;
    middle_0_r bend forward 55;
    middle_0_r sway right 0;
    middle_1_r bend forward 90;
    middle_2_r bend forward 35;
    ring_0_r bend forward 70;
    ring_0_r sway left 5;
    ring_1_r bend forward 90;
    ring_2_r bend forward 15;
    pinky_0_r bend forward 80;
    pinky_0_r sway right 5;
    pinky_1_r bend forward 90;
    pinky_2_r bend forward 15;
}

@pose pose_30 {
    elbow_r bend forward 100;
}

@pose pose_31 {
    thumb_1_r bend forward 10;
    thumb_2_r bend forward 20;
    index_0_r bend forward 10;
    index_0_r sway right 5;
    index_1_r bend forward 30;
    index_2_r bend forward 25;
    middle_0_r bend forward 10;
    middle_0_r sway left 5;
    middle_1_r bend forward 40;
    middle_2_r bend forward 25;
    ring_0_r bend forward 15;
    ring_0_r sway right 0;
    ring_1_r bend forward 40;
    ring_2_r bend forward 25;
    pinky_0_r bend forward 15;
    pinky_0_r sway left 5;
    pinky_1_r bend forward 30;
    pinky_2_r bend forward 40;
}

@pose pose_32 {
    wrist_r bend forward 55;
    wrist_r turn left 45;
    wrist_r sway right 40;
}

@pose pose_33 {
    wrist_r bend forward 10;
    wrist_r turn left 35;
    wrist_r sway left 0;
}

@pose pose_34 {
    center bend forward 0;
    center turn left 0;
    center sway left 0;
    upper_body bend forward 0;
    upper_body turn left 0;
    upper_body sway right 0;
    upper_body2 bend backward 0;
    upper_body2 turn right 0;
    upper_body2 sway left 0;
    lower_body bend backward 0;
    lower_body turn right 0;
    lower_body sway left 0;
    neck bend backward 0;
    neck turn right 0;
    neck sway left 0;
    head bend forward 0;
    head turn left 0;
    head sway right 0;
    shoulder_l bend forward 0;
    shoulder_l turn right 0;
    shoulder_l sway left 0;
    shoulder_r bend backward 0;
    shoulder_r turn left 0;
    shoulder_r sway right 0;
    arm_l bend forward 40;
    arm_l sway right 5;
    arm_r bend forward 40;
    arm_r turn left 0;
    arm_r sway right 5;
    elbow_l bend forward 15;
    elbow_r bend forward 15;
    wrist_l bend forward 5;
    wrist_l turn right 15;
    wrist_l sway left 5;
    wrist_r bend forward 15;
    wrist_r turn left 15;
    wrist_r sway left 0;
    leg_l bend forward 0;
    leg_l turn left 0;
    leg_l sway right 0;
    leg_r bend forward 0;
    leg_r turn right 0;
    leg_r sway right 0;
    thumb_1_l bend forward 10;
    thumb_2_l bend forward 20;
    index_0_l bend forward 10;
    index_0_l sway left 5;
    index_1_l bend forward 30;
    index_2_l bend forward 25;
    middle_0_l bend forward 10;
    middle_0_l sway right 5;
    middle_1_l bend forward 40;
    middle_2_l bend forward 25;
    ring_0_l bend forward 15;
    ring_0_l sway right 0;
    ring_1_l bend forward 40;
    ring_2_l bend forward 25;
    pinky_0_l bend forward 15;
    pinky_0_l sway right 5;
    pinky_1_l bend forward 30;
    pinky_2_l bend forward 40;
    thumb_1_r bend forward 10;
    thumb_2_r bend forward 20;
    index_0_r bend forward 10;
    index_0_r sway right 5;
    index_1_r bend forward 30;
    index_2_r bend forward 25;
    middle_0_r bend forward 10;
    middle_0_r sway left 5;
    middle_1_r bend forward 40;
    middle_2_r bend forward 25;
    ring_0_r bend forward 15;
    ring_0_r sway right 0;
    ring_1_r bend forward 40;
    ring_2_r bend forward 25;
    pinky_0_r bend forward 15;
    pinky_0_r sway left 5;
    pinky_1_r bend forward 30;
    pinky_2_r bend forward 40;
}

@animation extracted_animation {
    0.00: pose_0;
    0.33: pose_1;
    0.60: pose_2;
    0.70: pose_3;
    0.73: pose_4;
    0.87: pose_5;
    1.00: pose_6;
    1.03: pose_7;
    1.07: pose_8;
    1.17: pose_9;
    1.33: pose_10;
    1.63: pose_11;
    1.73: pose_12;
    2.33: pose_13;
    2.67: pose_14;
    2.97: pose_15;
    3.27: pose_16;
    4.00: pose_17;
    4.70: pose_18;
    4.80: pose_19;
    5.27: pose_20;
    5.33: pose_21;
    5.37: pose_22;
    5.53: pose_23;
    5.73: pose_24;
    5.83: pose_25;
    6.00: pose_26;
    6.10: pose_27;
    6.67: pose_28;
    6.83: pose_29;
    7.30: pose_30;
    7.40: pose_31;
    7.43: pose_32;
    7.80: pose_33;
    8.00: pose_34;
}

main {
    extracted_animation;
}`)

  const handleFileUpload = useCallback(
    async (event: React.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0]
      if (!file) return

      if (file.name.endsWith(".vpd")) {
        const url = URL.createObjectURL(file)
        const boneStates = await loadVPD(url)
        if (boneStates && mplCompiler) {
          const statements = mplCompiler.reverse_compile("vpd_pose", boneStates)
          setStatement(statements)
        }
      }

      event.target.value = ""
    },
    [setStatement, loadVPD, mplCompiler]
  )

  useEffect(() => {
    if (modelLoaded && mplCompiler) {
      try {
        const vmdBytes = mplCompiler.compile(statement)
        if (vmdBytes.length === 0) {
          loadVMD("")
          setVmdUrl(null)
          return
        }
        // Create a blob from the raw VMD bytes
        const vmdBlob = new Blob([vmdBytes], { type: "application/octet-stream" })
        const vmdUrl = URL.createObjectURL(vmdBlob)
        loadVMD(vmdUrl)
        setVmdUrl(vmdUrl)

        // Clean up the URL when component unmounts or statement changes
        return () => {
          URL.revokeObjectURL(vmdUrl)
        }
      } catch (error) {
        console.error(error)
      }
    }
  }, [statement, modelLoaded, mplCompiler, loadVMD])

  return (
    <div className="flex flex-col gap-1 w-full h-full">
      <div className="flex flex-row gap-2 px-6 pt-2 z-100 items-center justify-between">
        <h3 className="scroll-m-20 text-xl font-semibold tracking-tight hidden md:block">MMD Pose Language Editor</h3>
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
              <Upload className="size-4" />
              <span className="text-xs">Upload VPD</span>
            </Button>
          </div>

          <Button
            onClick={() => {
              if (vmdUrl) {
                const a = document.createElement("a")
                a.href = vmdUrl
                a.download = "animation.vmd"
                a.click()
              }
            }}
            className="flex"
            size="sm"
          >
            <Download className="size-4" />
            <span className="text-xs">Download VMD</span>
          </Button>

          <Button
            onClick={() => {
              setStatement("")
            }}
            className="flex gap-2 bg-black text-white hover:bg-black hover:text-white cursor-pointer"
            size="sm"
            variant="outline"
          >
            <RefreshCw className="size-4" />
          </Button>
          <Button
            size="sm"
            asChild
            className="bg-black text-white hover:bg-black hover:text-white px-2.5"
            variant="outline"
          >
            <Link href="https://github.com/AmyangXYZ/MPL" target="_blank" className="flex gap-2">
              <Image src="/github-mark-white.svg" alt="GitHub" width={18} height={18} />
            </Link>
          </Button>
        </div>
      </div>

      <div className="flex-1 py-2 px-6">
        <CodeEditor value={statement} onChange={setStatement} />
      </div>
    </div>
  )
}
