import { useEffect, useRef } from 'react';
import * as THREE from 'three';
import { riskPalette, type RiskBand } from '../theme';
import { useThreeScene } from './useThreeScene';

const turbulence: Record<RiskBand | 'idle', number> = { idle: 0.6, healthy: 0.5, watch: 0.9, at_risk: 1.3, breached: 1.8 };

const terrainVertex = /* glsl */ `
  uniform float uTime;
  uniform float uAmp;
  varying float vAlpha;
  varying float vHeight;
  void main() {
    vec3 p = position;
    float t = uTime;
    float h = sin(p.x * 0.14 + t * 0.55) * 1.1
            + sin(p.z * 0.21 + t * 0.4) * 0.8
            + cos((p.x + p.z) * 0.07 - t * 0.3) * 1.6;
    p.y = h * uAmp;
    vHeight = h;
    vec4 mv = modelViewMatrix * vec4(p, 1.0);
    gl_PointSize = clamp(140.0 / -mv.z, 1.5, 7.0);
    vAlpha = smoothstep(150.0, 25.0, -mv.z) * smoothstep(2.0, 12.0, -mv.z);
    gl_Position = projectionMatrix * mv;
  }
`;

const terrainFragment = /* glsl */ `
  uniform vec3 uColor;
  varying float vAlpha;
  varying float vHeight;
  void main() {
    float d = length(gl_PointCoord - 0.5);
    if (d > 0.5) discard;
    vec3 c = mix(uColor * 0.55, mix(uColor, vec3(1.0), 0.35), smoothstep(-2.0, 3.0, vHeight));
    gl_FragColor = vec4(c, vAlpha * (1.0 - d * 2.0) * 1.3);
  }
`;

/** Full-viewport particle horizon whose colour and turbulence follow the latest risk band. */
export function HorizonBackdrop({ risk }: { risk: RiskBand | 'idle' }) {
  const target = useRef({ color: new THREE.Color(riskPalette[risk].hex), amp: turbulence[risk] });

  useEffect(() => {
    target.current.color.setHex(riskPalette[risk].hex);
    target.current.amp = turbulence[risk];
  }, [risk]);

  const host = useThreeScene<HTMLDivElement>(({ scene, camera, pointer }) => {
    scene.fog = new THREE.FogExp2(0x05070f, 0.012);
    camera.position.set(0, 7, 34);

    const cols = 180;
    const rows = 90;
    const positions = new Float32Array(cols * rows * 3);
    for (let i = 0; i < cols; i++) {
      for (let j = 0; j < rows; j++) {
        const k = (i * rows + j) * 3;
        positions[k] = (i - cols / 2) * 1.1;
        positions[k + 2] = -j * 1.4 + 20;
      }
    }
    const terrainGeo = new THREE.BufferGeometry();
    terrainGeo.setAttribute('position', new THREE.BufferAttribute(positions, 3));
    const uniforms = {
      uTime: { value: 0 },
      uAmp: { value: target.current.amp },
      uColor: { value: target.current.color.clone() },
    };
    const terrain = new THREE.Points(
      terrainGeo,
      new THREE.ShaderMaterial({
        uniforms,
        vertexShader: terrainVertex,
        fragmentShader: terrainFragment,
        transparent: true,
        depthWrite: false,
        blending: THREE.AdditiveBlending,
      }),
    );
    scene.add(terrain);

    const starCount = 1400;
    const stars = new Float32Array(starCount * 3);
    for (let i = 0; i < starCount; i++) {
      stars[i * 3] = (Math.random() - 0.5) * 320;
      stars[i * 3 + 1] = 8 + Math.random() * 90;
      stars[i * 3 + 2] = -60 - Math.random() * 120;
    }
    const starGeo = new THREE.BufferGeometry();
    starGeo.setAttribute('position', new THREE.BufferAttribute(stars, 3));
    const starField = new THREE.Points(
      starGeo,
      new THREE.PointsMaterial({ color: 0xc7d2fe, size: 0.45, transparent: true, opacity: 0.55, depthWrite: false }),
    );
    scene.add(starField);

    const look = new THREE.Vector3(0, 6, -30);
    return {
      frame(elapsed, delta) {
        uniforms.uTime.value = elapsed;
        uniforms.uColor.value.lerp(target.current.color, delta * 1.5);
        uniforms.uAmp.value += (target.current.amp - uniforms.uAmp.value) * delta;
        starField.rotation.z = Math.sin(elapsed * 0.02) * 0.02;
        camera.position.x += (pointer.x * 4 - camera.position.x) * delta;
        camera.position.y += (7 + pointer.y * 2 - camera.position.y) * delta;
        camera.lookAt(look);
      },
    };
  }, 55);

  return <div ref={host} aria-hidden className="pointer-events-none fixed inset-0 -z-10" />;
}
