"use client"

import {
  ArcRotateCamera,
  Color3,
  Color4,
  CreateDisc,
  DirectionalLight,
  Engine,
  HemisphericLight,
  LoadAssetContainerAsync,
  Material,
  Mesh,
  RegisterSceneLoaderPlugin,
  Scene,
  ShadowGenerator,
  StandardMaterial,
  Vector3,
} from "@babylonjs/core"
import { useRef, useEffect, useCallback, useState } from "react"
import {
  MmdWasmModel,
  SdefInjector,
  MmdWasmInstanceTypeMPR,
  GetMmdWasmInstance,
  MmdWasmRuntime,
  MmdWasmPhysics,
  type IMmdWasmInstance,
  MmdStandardMaterialBuilder,
  MmdStandardMaterial,
  VpdLoader,
  MotionType,
  RigidBodyConstructionInfo,
  RigidBody,
  PhysicsStaticPlaneShape,
  BpmxLoader,
  VmdLoader,
  MmdPlayerControl,
} from "babylon-mmd"


import { MmdWasmPhysicsRuntimeImpl } from "babylon-mmd/esm/Runtime/Optimized/Physics/mmdWasmPhysicsRuntimeImpl"
import MPLInput from "./mpl-input"
import { MPLBoneFrame, Quaternion as MPLQuaternion, Vector3 as MPLVector3 } from "mmd-mpl"
import { useMPLCompiler } from "@/hooks/useMPLCompiler"


export default function MainScene() {
  const mplCompiler = useMPLCompiler()

  const canvasRef = useRef<HTMLCanvasElement>(null)
  const engineRef = useRef<Engine>(null)
  const sceneRef = useRef<Scene>(null)
  const shadowGeneratorRef = useRef<ShadowGenerator>(null)
  const mmdWasmInstanceRef = useRef<IMmdWasmInstance>(null)
  const mmdRuntimeRef = useRef<MmdWasmRuntime>(null)
  const mmdMaterialBuilderRef = useRef<MmdStandardMaterialBuilder>(null)
  const vpdLoaderRef = useRef<VpdLoader>(null)
  const vmdLoaderRef = useRef<VmdLoader>(null)
  const modelRef = useRef<MmdWasmModel>(null)

  const [modelLoaded, setModelLoaded] = useState(false)


  const loadModel = useCallback(async (): Promise<void> => {
    if (!sceneRef.current || !mmdWasmInstanceRef.current || !mmdRuntimeRef.current || !mplCompiler) return
    if (modelRef.current) {
      mmdRuntimeRef.current.destroyMmdModel(modelRef.current)
      modelRef.current.mesh.dispose()
    }

    LoadAssetContainerAsync(`/models/深空之眼-梵天.bpmx`, sceneRef.current!, {
      pluginOptions: {
        mmdmodel: {
          materialBuilder: mmdMaterialBuilderRef.current || undefined,
        },
      },
    }).then(async (result) => {
      const mesh = result.meshes[0]
      shadowGeneratorRef.current!.addShadowCaster(mesh)
      modelRef.current = mmdRuntimeRef.current!.createMmdModel(mesh as Mesh, {
        buildPhysics: {
          disableOffsetForConstraintFrame: true,
        },
      })

      result.addAllToScene()
      setModelLoaded(true)
    })
  }, [mplCompiler])

  const loadVMD = useCallback(
    async (vmdUrl: string) => {
      if (!vmdLoaderRef.current || !modelRef.current || !mplCompiler) return null
      if (vmdUrl === "") {
        modelRef.current.removeAnimation(0)
        return
      }
      const vmd = await vmdLoaderRef.current.loadAsync("vmd_animation", vmdUrl)
      modelRef.current.addAnimation(vmd)
      modelRef.current.setAnimation("vmd_animation")
      mmdRuntimeRef.current!.seekAnimation(0, true)
      mmdRuntimeRef.current!.playAnimation()
    },
    [vmdLoaderRef, modelRef, mplCompiler]
  )

  const loadVPD = useCallback(
    async (vpdUrl: string): Promise<MPLBoneFrame[] | null> => {
      if (!vpdLoaderRef.current || !modelRef.current || !mplCompiler) return null

      const vpd = await vpdLoaderRef.current.loadAsync("vpd_pose", vpdUrl)
      // modelRef.current.addAnimation(vpd)
      // modelRef.current.setAnimation("vpd_pose")
      // modelRef.current.currentAnimation?.animate(0)
      const boneStates: MPLBoneFrame[] = []
      for (const boneTrack of vpd.boneTracks) {
        const boneNameJp = boneTrack.name
        const boneNameEn = mplCompiler.get_bone_english_name(boneNameJp)
        if (!boneNameEn) {
          continue
        }

        const rotation = boneTrack.rotations
        if (rotation.length === 0) continue

        if (!(rotation[0] === 0 && rotation[1] === 0 && rotation[2] === 0 && rotation[3] === 1)) {
          boneStates.push(
            new MPLBoneFrame(
              boneNameEn,
              boneNameJp,
              new MPLVector3(0, 0, 0),
              new MPLQuaternion(rotation[0], rotation[1], rotation[2], rotation[3])
            )
          )
        }
      }

      for (const boneTrack of vpd.movableBoneTracks) {
        const boneNameJp = boneTrack.name
        const boneNameEn = mplCompiler.get_bone_english_name(boneNameJp)
        if (!boneNameEn) {
          continue
        }

        if (boneTrack.rotations && boneTrack.rotations.length > 0) {
          const rotation = boneTrack.rotations
          boneStates.push(
            new MPLBoneFrame(
              boneNameEn,
              boneNameJp,
              new MPLVector3(0, 0, 0),
              new MPLQuaternion(rotation[0], rotation[1], rotation[2], rotation[3])
            )
          )
        }
      }

      return boneStates
    },
    [vpdLoaderRef, modelRef, mplCompiler]
  )

  useEffect(() => {
    const resize = () => {
      if (sceneRef.current) {
        sceneRef.current.getEngine().resize()
      }
    }

    const init = async () => {
      if (!canvasRef.current || !mplCompiler) return

      // Register the PMX loader plugin
      RegisterSceneLoaderPlugin(new BpmxLoader())

      const engine = new Engine(canvasRef.current, true, {}, true)
      SdefInjector.OverrideEngineCreateEffect(engine)

      const scene = new Scene(engine)

      scene.clearColor = new Color4(0.99, 0.44, 0.66, 1.0)

      engineRef.current = engine
      sceneRef.current = scene

      const camera = new ArcRotateCamera("ArcRotateCamera", 0, 0, 45, new Vector3(0, 12, 0), scene)
      camera.setPosition(new Vector3(0, 19, -25))
      camera.attachControl(canvasRef.current, false)
      camera.inertia = 0.8
      camera.speed = 10

      scene.activeCameras = [camera]

      const hemisphericLight = new HemisphericLight("hemisphericLight", new Vector3(0, 1, 0), scene)
      hemisphericLight.intensity = 0.5
      hemisphericLight.specular = new Color3(0, 0, 0)
      hemisphericLight.groundColor = new Color3(1, 1, 1)

      const directionalLight = new DirectionalLight("directionalLight", new Vector3(2, -19.15, 4), scene)
      directionalLight.intensity = 0.9

      const shadowGenerator = new ShadowGenerator(2048, directionalLight)
      shadowGeneratorRef.current = shadowGenerator

      mmdWasmInstanceRef.current = await GetMmdWasmInstance(new MmdWasmInstanceTypeMPR())
      const mmdRuntime = new MmdWasmRuntime(mmdWasmInstanceRef.current, scene, new MmdWasmPhysics(scene))
      mmdRuntime.register(scene)
      mmdRuntimeRef.current = mmdRuntime

      const ground = CreateDisc("stageGround", { radius: 12, tessellation: 64 }, scene)
      const groundMaterial = new StandardMaterial("groundMaterial", scene)
      groundMaterial.diffuseColor = new Color3(0.95, 0.98, 1.0)
      groundMaterial.emissiveColor = new Color3(0.1, 0.15, 0.25)
      groundMaterial.specularColor = new Color3(0.2, 0.3, 0.5)
      ground.material = groundMaterial
      ground.rotation.x = Math.PI / 2
      ground.receiveShadows = true

      const physicsRuntime = mmdRuntime.physics!.getImpl(MmdWasmPhysicsRuntimeImpl)
      {
        const info = new RigidBodyConstructionInfo(mmdRuntime.wasmInstance)
        info.motionType = MotionType.Static
        info.shape = new PhysicsStaticPlaneShape(physicsRuntime, new Vector3(0, 0.5, 0), 0)
        const groundBody = new RigidBody(physicsRuntime, info)
        physicsRuntime.addRigidBodyToGlobal(groundBody)
      }

      const materialBuilder = new MmdStandardMaterialBuilder()
      //   materialBuilder.loadOutlineRenderingProperties = (): void => {
      //     /* do nothing */
      //   }
      materialBuilder.afterBuildSingleMaterial = (material: MmdStandardMaterial): void => {
        material.forceDepthWrite = true
        material.useAlphaFromDiffuseTexture = true
        material.specularColor = new Color3(0, 0, 0)
        if (material.diffuseTexture !== null) material.diffuseTexture.hasAlpha = true

        if (material.transparencyMode === Material.MATERIAL_ALPHABLEND) {
          material.transparencyMode = Material.MATERIAL_ALPHATESTANDBLEND
          material.alphaCutOff = 0.01
        }
      }
      mmdMaterialBuilderRef.current = materialBuilder

      vpdLoaderRef.current = new VpdLoader(scene)
      vmdLoaderRef.current = new VmdLoader(scene)
      const playerControl = new MmdPlayerControl(scene, mmdRuntime)
      playerControl.showPlayerControl()

      loadModel()

      window.addEventListener("resize", resize)

      engine.runRenderLoop(() => {
        scene.render()
      })
    }
    init()

    return () => {
      if (engineRef.current) {
        engineRef.current.dispose()
        window.removeEventListener("resize", resize)
      }
    }
  }, [loadModel, mplCompiler])

  return (
    <div className="w-full h-full flex flex-col md:flex-row">
      <div className="w-full h-[70%] md:w-1/2 md:h-full order-1 md:order-2">
        <canvas ref={canvasRef} className="w-full h-full z-1" />
      </div>
      <div className="w-full h-[30%] md:w-1/2 md:h-full order-2 md:order-1 border-t">
        <MPLInput loadVPD={loadVPD} modelLoaded={modelLoaded} loadVMD={loadVMD} />
      </div>
    </div>
  )
}
