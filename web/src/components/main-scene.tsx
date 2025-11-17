"use client"


import { useCallback, useEffect, useRef, useState } from "react"
import { Engine } from "reze-engine"
import MPLInput from "./mpl-input"

export default function MainScene() {

  const canvasRef = useRef<HTMLCanvasElement>(null)
  const engineRef = useRef<Engine>(null)
  const [engineError, setEngineError] = useState<string | null>(null)
  const [modelLoaded, setModelLoaded] = useState(false)

  const loadVMD = useCallback(async (url: string) => {
    await engineRef.current?.loadAnimation(url)
    engineRef.current?.playAnimation()
  }, [])

  const initEngine = useCallback(async () => {
    if (canvasRef.current) {
      // Initialize engine
      try {
        const engine = new Engine(canvasRef.current, {
          ambient: 0.96,
          rimLightIntensity: 0.2,
          bloomIntensity: 0.06,
        })
        engineRef.current = engine
        await engine.init()
        await engine.loadModel("/models/深空之眼-梵天/深空之眼-梵天-short-hair.pmx")

        engine.runRenderLoop(() => {
        })
        setTimeout(() => setModelLoaded(true), 200)


      } catch (error) {
        setEngineError(error instanceof Error ? error.message : "Unknown error")
      }
    }
  }, [])

  useEffect(() => {
    void (async () => {
      initEngine()
    })()

    // Cleanup on unmount
    return () => {
      if (engineRef.current) {
        engineRef.current.dispose()
      }
    }
  }, [initEngine])



  return (
    <div className="w-full h-full flex flex-col md:flex-row">
      <div className="w-full h-[70%] md:w-1/2 md:h-full order-1 md:order-2 bg-rose-400">
        <canvas ref={canvasRef} className="w-full h-full z-1" />
        {engineError && <div className="text-red-500 text-center">{engineError}</div>}
      </div>
      <div className="w-full h-[30%] md:w-1/2 md:h-full order-2 md:order-1 border-t">
        <MPLInput modelLoaded={modelLoaded} loadVMD={loadVMD} />
      </div>
    </div>
  )
}
