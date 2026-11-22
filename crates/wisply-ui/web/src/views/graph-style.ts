import Sigma from "sigma";
import { DEPTHLESS_STYLES, type RenderParams } from "sigma/types";
import type { Net } from "./graph-data";
import { EDGE_FLAGS, GLOW_FLAGS, NODE_FLAGS, type Glow } from "./graph-light";

export const LABEL_MIN = 7;
const LIFT_MAX = 4;
const LABEL_SIZE = 13;
const LIT_EDGE = 1.6;
const FONT = "Segoe UI Variable Text, Segoe UI, system-ui, sans-serif";
const VERTEX = "#version 300 es\nvoid main(){gl_Position=vec4(float((gl_VertexID<<1)&2)*2.0-1.0,float(gl_VertexID&2)*2.0-1.0,0.0,1.0);}";
const FRAGMENT = "#version 300 es\nprecision mediump float;uniform vec4 u_tint;out vec4 o_color;void main(){o_color=u_tint;}";

export type Tones = { label: string; dim: string; edge: string; edgeOn: string; back: string; backLine: string; shadow: string };
export type Rgba = [number, number, number, number];

export function rgba(text: string): Rgba {
  const s = text.trim().toLowerCase();
  if (s.startsWith("#")) {
    const x = s.slice(1);
    const wide = x.length <= 4 ? [...x].map((c) => c + c).join("") : x;
    const at = (i: number) => parseInt(wide.slice(i, i + 2), 16) / 255;
    return [at(0), at(2), at(4), wide.length >= 8 ? at(6) : 1];
  }
  const parts = (s.match(/[\d.]+%?/g) ?? []).map((p) => (p.endsWith("%") ? parseFloat(p) / 100 : parseFloat(p)));
  const [r = 0, g = 0, b = 0, a = 1] = parts;
  return [r / 255, g / 255, b / 255, a];
}

export function lift(edge: Rgba): number {
  return Math.min(LIFT_MAX, Math.max(0, Math.ceil(Math.log2(1 / Math.max(edge[3], 2 ** -LIFT_MAX)))));
}

export function veil(tint: Rgba, doubles: number, on: () => boolean) {
  return (gl: WebGL2RenderingContext) => {
    const program = gl.createProgram();
    for (const [type, source] of [[gl.VERTEX_SHADER, VERTEX], [gl.FRAGMENT_SHADER, FRAGMENT]] as const) {
      const shader = gl.createShader(type) as WebGLShader;
      gl.shaderSource(shader, source);
      gl.compileShader(shader);
      gl.attachShader(program, shader);
      gl.deleteShader(shader);
    }
    gl.linkProgram(program);
    const at = gl.getUniformLocation(program, "u_tint");
    const shape = gl.createVertexArray();
    return {
      render(p: RenderParams) {
        if (!on()) return;
        gl.bindFramebuffer(gl.FRAMEBUFFER, null);
        gl.viewport(0, 0, p.width * p.pixelRatio, p.height * p.pixelRatio);
        gl.enable(gl.BLEND);
        gl.useProgram(program);
        gl.bindVertexArray(shape);
        gl.blendFuncSeparate(gl.ZERO, gl.ONE, gl.DST_ALPHA, gl.ONE);
        gl.uniform4f(at, 0, 0, 0, 1);
        for (let i = 0; i < doubles; i++) gl.drawArrays(gl.TRIANGLES, 0, 3);
        gl.blendFuncSeparate(gl.DST_ALPHA, gl.ZERO, gl.DST_ALPHA, gl.ZERO);
        gl.uniform4f(at, tint[0] * tint[3], tint[1] * tint[3], tint[2] * tint[3], tint[3]);
        gl.drawArrays(gl.TRIANGLES, 0, 3);
        gl.bindVertexArray(null);
        gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
      },
      kill() {
        gl.deleteProgram(program);
        gl.deleteVertexArray(shape);
      },
    };
  };
}

function text(t: Tones) {
  return { labelColor: t.label, labelFont: FONT, labelSize: LABEL_SIZE } as const;
}

function settings() {
  return {
    labelRenderedSizeThreshold: LABEL_MIN,
    labelDensity: 0.35,
    labelGridCellSize: 110,
    minEdgeThickness: 0.8,
    zoomToSizeRatioFunction: () => 1,
  };
}

export function rules(t: Tones) {
  const nodes = [
    DEPTHLESS_STYLES.nodes,
    { ...text(t), label: { attribute: "tag" }, cursor: "pointer", depth: "nodes" } as const,
    { whenState: "named", then: { labelVisibility: "visible" } } as const,
    { whenState: "mute", then: { labelVisibility: "hidden" } } as const,
    { whenState: "off", then: { visibility: "hidden" } } as const,
  ];
  const edges = [DEPTHLESS_STYLES.edges, { color: t.edge, depth: "edges" } as const, { whenState: "off", then: { visibility: "hidden" } } as const];
  return { nodes, edges };
}

export function glowRules(t: Tones) {
  const nodes = [
    DEPTHLESS_STYLES.nodes,
    {
      ...text(t),
      labelVisibility: "visible",
      backdropVisibility: "visible",
      backdropColor: t.back,
      backdropBorderColor: t.backLine,
      backdropBorderWidth: 1,
      backdropCornerRadius: 6,
      backdropShadowColor: t.shadow,
      depth: "nodes",
    } as const,
    { whenData: "pin", then: { depth: "pin" } } as const,
    { whenState: "hush", then: { labelVisibility: "hidden", backdropVisibility: "hidden" } } as const,
  ];
  const edges = [DEPTHLESS_STYLES.edges, { color: t.edgeOn, size: LIT_EDGE, depth: "edges" } as const];
  return { nodes, edges };
}

export function stageOptions(t: Tones) {
  return { primitives: { depthLayers: ["edges", "nodes", "veil"] as ["edges", "nodes", "veil"] }, customNodeState: { ...NODE_FLAGS }, customEdgeState: { ...EDGE_FLAGS }, styles: rules(t), settings: settings() };
}

export function crownOptions(t: Tones) {
  return { primitives: { depthLayers: ["edges", "nodes", "pin"] as ["edges", "nodes", "pin"] }, customNodeState: { ...GLOW_FLAGS }, styles: glowRules(t), settings: settings() };
}

export function stage(g: Net, canvas: HTMLElement, t: Tones) {
  return new Sigma(g, canvas, stageOptions(t));
}

export function crown(top: Glow, canvas: HTMLElement, t: Tones) {
  return new Sigma(top, canvas, crownOptions(t));
}

export type Stage = ReturnType<typeof stage>;
export type Crown = ReturnType<typeof crown>;
