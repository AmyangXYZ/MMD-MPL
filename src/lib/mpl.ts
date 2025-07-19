import { Morphs, MovableBones, Pose, RotatableBones } from "./pose"

export const BONES: Record<string, string> = {
  base: "全ての親",
  center: "センター",
  upper_body: "上半身",
  lower_body: "下半身",
  waist: "腰",
  neck: "首",
  head: "頭",
  shoulder_l: "左肩",
  shoulder_r: "右肩",
  arm_l: "左腕",
  arm_r: "右腕",
  arm_twist_l: "左腕捩",
  arm_twist_r: "右腕捩",
  elbow_l: "左ひじ",
  elbow_r: "右ひじ",
  wrist_l: "左手首",
  wrist_r: "右手首",
  wrist_twist_l: "左手捩",
  wrist_twist_r: "右手捩",
  leg_l: "左足",
  leg_r: "右足",
  knee_l: "左ひざ",
  knee_r: "右ひざ",
  ankle_l: "左足首",
  ankle_r: "右足首",
  ankle_ex_l: "左足先EX",
  ankle_ex_r: "右足先EX",
  eye_l: "左目",
  eye_r: "右目",
  thumb_l_0: "左親指０",
  thumb_r_0: "右親指０",
  thumb_l_1: "左親指１",
  thumb_r_1: "右親指１",
  thumb_l_2: "左親指２",
  thumb_r_2: "右親指２",
  index_l_0: "左人指１",
  index_r_0: "右人指１",
  index_l_1: "左人指２",
  index_r_1: "右人指２",
  index_l_2: "左人指３",
  index_r_2: "右人指３",
  middle_l_0: "左中指１",
  middle_r_0: "右中指１",
  middle_l_1: "左中指２",
  middle_r_1: "右中指２",
  middle_l_2: "左中指３",
  middle_r_2: "右中指３",
  ring_l_0: "左薬指１",
  ring_r_0: "右薬指１",
  ring_l_1: "左薬指２",
  ring_r_1: "右薬指２",
  ring_l_2: "左薬指３",
  ring_r_2: "右薬指３",
  pinky_l_0: "左小指１",
  pinky_r_0: "右小指１",
  pinky_l_1: "左小指２",
  pinky_r_1: "右小指２",
  pinky_l_2: "左小指３",
  pinky_r_2: "右小指３",
}

export const ACTIONS: string[] = ["bend", "turn", "twist"]

export const DIRECTIONS: string[] = ["inward", "outward", "left", "right"]

export const BONE_ACTIONS: Record<string, string[]> = {
  // Core body
  base: ["bend", "turn", "twist"],
  center: ["bend", "turn", "twist"],
  head: ["bend", "turn", "twist"],
  neck: ["bend", "turn", "twist"],
  upper_body: ["bend", "turn", "twist"],
  lower_body: ["bend", "turn", "twist"],
  waist: ["bend", "turn", "twist"],

  // Arms & shoulders
  shoulder_l: ["bend", "turn", "twist"],
  shoulder_r: ["bend", "turn", "twist"],
  arm_l: ["bend", "turn", "twist"],
  arm_r: ["bend", "turn", "twist"],
  arm_twist_l: ["twist"],
  arm_twist_r: ["twist"],
  elbow_l: ["bend"],
  elbow_r: ["bend"],
  wrist_l: ["bend", "turn"],
  wrist_r: ["bend", "turn"],
  wrist_twist_l: ["twist"],
  wrist_twist_r: ["twist"],

  // Legs
  leg_l: ["bend", "turn", "twist"],
  leg_r: ["bend", "turn", "twist"],
  knee_l: ["bend"],
  knee_r: ["bend"],
  ankle_l: ["bend", "turn"],
  ankle_r: ["bend", "turn"],
  ankle_ex_l: ["bend"],
  ankle_ex_r: ["bend"],

  // Eyes
  eye_l: ["turn"],
  eye_r: ["turn"],

  // Fingers - mainly bend, limited turn/twist
  thumb_l_0: ["bend", "turn"],
  thumb_r_0: ["bend", "turn"],
  thumb_l_1: ["bend"],
  thumb_r_1: ["bend"],
  thumb_l_2: ["bend"],
  thumb_r_2: ["bend"],

  index_l_0: ["bend", "turn"],
  index_r_0: ["bend", "turn"],
  index_l_1: ["bend"],
  index_r_1: ["bend"],
  index_l_2: ["bend"],
  index_r_2: ["bend"],

  middle_l_0: ["bend", "turn"],
  middle_r_0: ["bend", "turn"],
  middle_l_1: ["bend"],
  middle_r_1: ["bend"],
  middle_l_2: ["bend"],
  middle_r_2: ["bend"],

  ring_l_0: ["bend", "turn"],
  ring_r_0: ["bend", "turn"],
  ring_l_1: ["bend"],
  ring_r_1: ["bend"],
  ring_l_2: ["bend"],
  ring_r_2: ["bend"],

  pinky_l_0: ["bend", "turn"],
  pinky_r_0: ["bend", "turn"],
  pinky_l_1: ["bend"],
  pinky_r_1: ["bend"],
  pinky_l_2: ["bend"],
  pinky_r_2: ["bend"],
}

export const BONE_LIMITS: Record<string, Record<string, Record<string, number>>> = {
  // Core body
  head: {
    bend: { inward: 60, outward: 90 }, // nod down/up
    turn: { left: 90, right: 90 }, // look left/right
    twist: { left: 45, right: 45 }, // tilt ear to shoulder
  },
  neck: {
    bend: { inward: 45, outward: 60 },
    turn: { left: 75, right: 75 },
    twist: { left: 30, right: 30 },
  },
  upper_body: {
    bend: { inward: 30, outward: 45 },
    turn: { left: 45, right: 45 },
    twist: { left: 30, right: 30 },
  },

  // Arms & shoulders
  shoulder_l: {
    bend: { inward: 45, outward: 180 }, // arms down/up
    turn: { left: 30, right: 45 },
    twist: { left: 45, right: 45 },
  },
  shoulder_r: {
    bend: { inward: 45, outward: 180 },
    turn: { left: 45, right: 30 },
    twist: { left: 45, right: 45 },
  },
  arm_l: {
    bend: { inward: 45, outward: 180 },
    turn: { left: 90, right: 45 },
    twist: { left: 90, right: 90 },
  },
  arm_r: {
    bend: { inward: 45, outward: 180 },
    turn: { left: 45, right: 90 },
    twist: { left: 90, right: 90 },
  },
  elbow_l: {
    bend: { inward: 150, outward: 0 }, // elbow only bends inward
  },
  elbow_r: {
    bend: { inward: 150, outward: 0 },
  },
  wrist_l: {
    bend: { inward: 90, outward: 70 }, // flex/extend
    turn: { left: 20, right: 35 }, // limited side bend
  },
  wrist_r: {
    bend: { inward: 90, outward: 70 },
    turn: { left: 35, right: 20 },
  },
  wrist_twist_l: {
    twist: { left: 90, right: 90 }, // forearm rotation
  },
  wrist_twist_r: {
    twist: { left: 90, right: 90 },
  },

  // Legs
  knee_l: {
    bend: { inward: 135, outward: 0 }, // knee only bends inward
  },
  knee_r: {
    bend: { inward: 135, outward: 0 },
  },
  ankle_l: {
    bend: { inward: 50, outward: 30 }, // point/flex foot
    turn: { left: 25, right: 25 },
  },
  ankle_r: {
    bend: { inward: 50, outward: 30 },
    turn: { left: 25, right: 25 },
  },

  // Eyes
  eye_l: {
    turn: { left: 45, right: 15 }, // limited eye movement
  },
  eye_r: {
    turn: { left: 15, right: 45 },
  },

  // Fingers - example for thumbs
  thumb_l_0: {
    bend: { inward: 60, outward: 15 },
    turn: { left: 45, right: 45 }, // thumb spread
  },
  thumb_r_0: {
    bend: { inward: 60, outward: 15 },
    turn: { left: 45, right: 45 },
  },
  thumb_l_1: {
    bend: { inward: 90, outward: 0 }, // thumb curl only
  },
  thumb_r_1: {
    bend: { inward: 90, outward: 0 },
  },
  index_l_0: {
    bend: { inward: 60, outward: 15 },
    turn: { left: 45, right: 45 },
  },
  index_r_0: {
    bend: { inward: 60, outward: 15 },
  },
  index_l_1: {
    bend: { inward: 90, outward: 0 },
  },
  index_r_1: {
    bend: { inward: 90, outward: 0 },
  },
  index_l_2: {
    bend: { inward: 90, outward: 0 },
  },
  index_r_2: {
    bend: { inward: 90, outward: 0 },
  },
  middle_l_0: {
    bend: { inward: 60, outward: 15 },
  },
  middle_r_0: {
    bend: { inward: 60, outward: 15 },
  },
  middle_l_1: {
    bend: { inward: 90, outward: 0 },
  },
  middle_r_1: {
    bend: { inward: 90, outward: 0 },
  },
  middle_l_2: {
    bend: { inward: 90, outward: 0 },
  },
  middle_r_2: {
    bend: { inward: 90, outward: 0 },
  },
  ring_l_0: {
    bend: { inward: 60, outward: 15 },
  },
  ring_r_0: {
    bend: { inward: 60, outward: 15 },
  },
  ring_l_1: {
    bend: { inward: 90, outward: 0 },
  },
  ring_r_1: {
    bend: { inward: 90, outward: 0 },
  },
  ring_l_2: {
    bend: { inward: 90, outward: 0 },
  },
  ring_r_2: {
    bend: { inward: 90, outward: 0 },
  },
  pinky_l_0: {
    bend: { inward: 60, outward: 15 },
  },
  pinky_r_0: {
    bend: { inward: 60, outward: 15 },
  },
  pinky_l_1: {
    bend: { inward: 90, outward: 0 },
  },
  pinky_r_1: {
    bend: { inward: 90, outward: 0 },
  },
  pinky_l_2: {
    bend: { inward: 90, outward: 0 },
  },
  pinky_r_2: {
    bend: { inward: 90, outward: 0 },
  },
}

export const MPLInterpreter = (description: string): Pose | null => {
  if (description === "") {
    return null
  }
  const segments = description.split(" ")
  if (segments.length !== 4) {
    return null
  }
  const bone = segments[0] as string
  const action = segments[1] as string
  const direction = segments[2] as string
  const degrees = Number(segments[3])
  if (!BONES[bone] || !ACTIONS.includes(action) || !DIRECTIONS.includes(direction) || isNaN(degrees)) {
    return null
  }
  if (degrees > BONE_LIMITS[bone][action][direction]) {
    return null
  }
  const pose: Pose = {
    description: description,
    face: {} as Morphs,
    movableBones: {} as MovableBones,
    rotatableBones: {} as RotatableBones,
  }
  return pose
}
