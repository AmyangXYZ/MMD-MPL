import { useCallback, useState, useEffect } from "react"
import { Button } from "./ui/button"
import { Import, RefreshCw } from "lucide-react"
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

  const [statement, setStatement] = useState(`@pose default {
               head turn right 0;
               head sway right 0;
               waist bend forward 0;
}

        @pose look_left {
               head turn left 20;
               head sway left 20;
}

       @pose look_right {
               head turn right 20;
               head sway right 20;
}

       @pose bend_over {
               waist bend forward 90;
}
                      @pose stand {
               waist bend forward 0;
}
               

       @animation look_around {
      
  0.8: look_right;
  0.9: look_left;
  1.2: look_right;
}

@animation bow {
    1.4: stand & look_left;
  2.0: bend_over;
  2.3: stand;
    }

    @animation default1 {
        0.5: bend_over;
    }

  main {
        default1;
    
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
          return
        }
        // Create a blob from the raw VMD bytes
        const vmdBlob = new Blob([vmdBytes], { type: 'application/octet-stream' })
        const vmdUrl = URL.createObjectURL(vmdBlob)
        loadVMD(vmdUrl)

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
        <CodeEditor
          value={statement}
          onChange={setStatement}
        />
      </div>
    </div>
  )
}
