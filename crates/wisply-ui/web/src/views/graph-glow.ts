import type { FragmentLayer } from "sigma/rendering";
import { DEFAULT_NODE_PRIMITIVES } from "sigma/types";

export const TAU = Math.PI * 2;
export const HALO = 4;
const GLOW = 1.3;
const FALLOFF = 1.9;
const OCCLUDE = 0.4;
const HOT = 0.65;
const SOFT = 0.55;
const SWAY = 0.3;
const PICK = 1.4;
const PHASE = "u_phase";
const GLOW_PX = 5.5;
const REF_SIZE = 1.6;
const FIT = 0.9;
const REACH_MAX = 400;

export const clock = { phase: 0 };

export function reach(extent: number, view: number): number {
  const px = (FIT * Math.max(view, 1)) / Math.max(extent, 1);
  return Math.min(REACH_MAX, Math.max(HALO, GLOW_PX / (px * REF_SIZE)));
}

const num = (n: number) => (Number.isInteger(n) ? n.toFixed(1) : String(n));

export function halo(): FragmentLayer {
  const core = num(1 / HALO);
  const glsl = `
uniform float ${PHASE};
vec4 layer_halo(vec4 v_fillColor) {
  float d = length(context.uv);
  float own = fract(sin(dot(v_id.xyz, vec3(12.9898, 78.233, 37.719))) * 43758.5453);
  float sway = 1.0 + ${num(SWAY)} * sin(${PHASE} + own * ${num(TAU)});
  float body = 1.0 - smoothstep(${core} * ${num(SOFT)}, ${core}, d);
  float aura = pow(max(0.0, 1.0 - d), ${num(FALLOFF)}) * ${num(GLOW)} * sway;
  vec3 base = v_fillColor.rgb;
  vec3 hot = mix(base, vec3(1.0), ${num(HOT)});
  vec3 lit = min(hot * body + base * aura * (1.0 - body), vec3(1.0));
  float a = max(body, aura * ${num(OCCLUDE)});
  #ifdef PICKING_MODE
  if (d > ${core} * ${num(PICK)}) discard;
  #endif
  return vec4(lit / max(a, 0.0001), a * v_fillColor.a);
}`;
  return {
    name: "halo",
    uniforms: [],
    attributes: [{ name: "fillColor", size: 4, type: WebGL2RenderingContext.UNSIGNED_BYTE, normalized: true, source: "color" }],
    glsl,
    lifecycle: (ctx) => {
      let at: WebGLUniformLocation | null = null;
      return {
        beforeRender() {
          at ??= ctx.getUniformLocation(PHASE);
          if (at) ctx.gl.uniform1f(at, clock.phase);
        },
      };
    },
  };
}

export function galaxy() {
  return { ...DEFAULT_NODE_PRIMITIVES, layers: [halo()] };
}
