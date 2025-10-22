import { useCallback, useState, useEffect, useRef } from "react"
import { Button } from "./ui/button"
import { Download, RefreshCw, Upload } from "lucide-react"
import Link from "next/link"
import Image from "next/image"
import { useMPLCompiler } from "@/hooks/useMPLCompiler"
import CodeEditor from "./code-editor"
import { FilesetResolver, HolisticLandmarker } from "@mediapipe/tasks-vision"
import { Solver } from "@/lib/mediapipe_solver"

export default function MPLInput({
  modelLoaded,
  loadVMD,
}: {
  loadVMD: (url: string) => void
  modelLoaded: boolean
}) {
  const mplCompiler = useMPLCompiler()
  const [vmdUrl, setVmdUrl] = useState<string | null>(null)
  const [compileError, setCompileError] = useState<string | null>(null)

  const [statement, setStatement] = useState(`@pose stand {
    center sway left 5, turn right 5, bend forward 5;
    upper_body sway right 5, bend backward 5;
    lower_body turn left 5;
    neck turn left 10, bend forward 10, sway right 5;
    head turn left 20, bend forward 20;
    shoulder_l turn right 5, sway left 10, bend backward 20;
    shoulder_r turn right 5, bend backward 10, sway left 10;
    arm_l bend forward 60;
    arm_r bend forward 45;
    elbow_l bend forward 15;
    elbow_r bend forward 15;
    wrist_l sway left 15;
    wrist_r turn left 5, bend backward 10, sway right 15;
    leg_l turn left 10;
    leg_r turn right 5, bend forward 20, sway left 10;
    knee_l bend backward 5;
    knee_r bend backward 5;
    ankle_l bend backward 15, sway left 5;
    ankle_r bend forward 5, turn left 10, sway right 5;
    toe_l bend forward 5;
    toe_r bend forward 5;
}

@pose hand_relax {
    thumb_l bend forward 10;
    index_l bend forward 45;
    middle_l sway right 5, bend forward 55;
    ring_l sway right 5, bend forward 55;
    pinky_l bend forward 60, sway right 5;
    thumb_r bend forward 10, sway left 5;
    index_r sway right 5, bend forward 35;
    middle_r sway right 5, bend forward 50;
    ring_r sway left 5, bend forward 60;
    pinky_r sway left 10, bend forward 55;
}

@pose kick {
    leg_l bend forward 120;
    knee_l bend backward 10;
}

@pose look {
    head reset;
    neck reset;
}

@animation hello {
    0: stand & hand_relax;
    1: kick;
    1.2: look;
}

main {
    hello;
}
`)

  const holisticLandmarkerRef = useRef<HolisticLandmarker | null>(null)
  const imageRef = useRef<HTMLImageElement | null>(null)

  const detectLandmarks = useCallback(async (): Promise<Blob | null> => {
    if (!holisticLandmarkerRef.current) {
      const vision = await FilesetResolver.forVisionTasks("https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@latest/wasm")
      holisticLandmarkerRef.current = await HolisticLandmarker.createFromOptions(vision, {
        baseOptions: {
          modelAssetPath:
            "https://storage.googleapis.com/mediapipe-models/holistic_landmarker/holistic_landmarker/float16/latest/holistic_landmarker.task",
          delegate: "GPU",
        },
        runningMode: "IMAGE",
      })
    }

    await holisticLandmarkerRef.current?.setOptions({ runningMode: "IMAGE" })

    if (
      imageRef.current &&
      imageRef.current.src.length > 0 &&
      imageRef.current.complete &&
      imageRef.current.naturalWidth > 0
    ) {
      let vpdBlob: Blob | null = null
      holisticLandmarkerRef.current!.detect(imageRef.current, (result) => {
        if (result.poseWorldLandmarks.length > 0) {
          const solver = new Solver()
          solver.solve(result)
          vpdBlob = solver.exportToVpdBlob("pose_from_image")
        }
      })
      return vpdBlob
    }
    return null
  }, [])

  const handleFileUpload = useCallback(
    async (event: React.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0]
      if (!file) return

      if (file.name.endsWith(".vpd")) {
        if (mplCompiler) {
          try {
            const statements = mplCompiler.reverse_compile("vpd", new Uint8Array(await file.arrayBuffer()))
            setStatement(statements)
          } catch (error) {
            console.error(error)
          }
        }
      } else if (file.name.endsWith(".vmd")) {
        if (mplCompiler) {
          try {
            const statements = mplCompiler.reverse_compile("vmd", new Uint8Array(await file.arrayBuffer()))
            setStatement(statements)
          } catch (error) {
            console.error(error)
          }
        }
      } else if (
        file.name.endsWith(".png") ||
        file.name.endsWith(".jpg") ||
        file.name.endsWith(".jpeg") ||
        file.name.endsWith(".webp")
      ) {
        const image = new window.Image()
        image.src = URL.createObjectURL(file)
        image.onload = async () => {
          imageRef.current = image
          const vpdBlob = await detectLandmarks()
          if (vpdBlob && mplCompiler) {
            try {
              const statements = mplCompiler.reverse_compile("vpd", new Uint8Array(await vpdBlob.arrayBuffer()))
              setStatement(statements)
            } catch (error) {
              console.error(error)
            }
          }
        }
        event.target.value = ""
      }
    },
    [setStatement, mplCompiler, detectLandmarks]
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
        setCompileError(null)
        // Create a blob from the raw VMD bytes
        const vmdBlob = new Blob([new Uint8Array(vmdBytes)], { type: "application/octet-stream" })
        const vmdUrl = URL.createObjectURL(vmdBlob)
        loadVMD(vmdUrl)
        setVmdUrl(vmdUrl)

        // Clean up the URL when component unmounts or statement changes
        return () => {
          URL.revokeObjectURL(vmdUrl)
        }
      } catch (error) {
        setCompileError(error as string)
      }
    }
  }, [statement, modelLoaded, mplCompiler, loadVMD])

  return (
    <div className="flex flex-col gap-1 w-full h-full">
      <div className="flex flex-row gap-2 px-6 pt-2 z-100 items-center justify-between">
        <h3 className="scroll-m-20 text-xl font-semibold tracking-tight">MPL Editor</h3>
        <div className="flex flex-row gap-2">
          <div className="relative hidden md:block">
            <input
              type="file"
              accept=".vpd,.vmd, .png, .jpg, .jpeg, .webp"
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
              <span className="text-xs">Upload Image/VPD/VMD</span>
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
        {compileError && <div className="text-red-500 text-sm font-mono mt-1">{compileError}</div>}
      </div>
    </div>
  )
}
