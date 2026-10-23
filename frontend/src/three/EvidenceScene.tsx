import { useEffect, useRef } from 'react';
import * as THREE from 'three';
import type { Evaluation } from '../client';
import { riskPalette } from '../theme';
import { glowTexture, useThreeScene } from './useThreeScene';

type Props = { points: number[]; target: number; evaluation: Evaluation | null };

const WIDTH = 12;
const HEIGHT = 6;

function buildData(group: THREE.Group, glow: THREE.Texture, { points, target, evaluation }: Props) {
  group.traverse((obj) => {
    const mesh = obj as THREE.Mesh;
    if (obj === group) return;
    mesh.geometry?.dispose();
    (mesh.material as THREE.Material | undefined)?.dispose();
  });
  group.clear();
  const color = new THREE.Color(riskPalette[evaluation?.risk_band ?? 'idle'].hex);
  const floor = Math.max(0, Math.min(...points, target, evaluation?.lower_bound ?? 1) - 0.04);
  const y = (v: number) => ((v - floor) / (1 - floor || 1)) * HEIGHT - HEIGHT / 2;
  const x = (i: number) => (points.length < 2 ? 0 : (i / (points.length - 1)) * WIDTH - WIDTH / 2);

  // Commitment target: a solid luminous sheet the trajectory must stay above.
  const targetSheet = new THREE.Mesh(
    new THREE.PlaneGeometry(WIDTH + 3, 5),
    new THREE.MeshBasicMaterial({ color: 0x5eead4, transparent: true, opacity: 0.09, side: THREE.DoubleSide, depthWrite: false }),
  );
  targetSheet.rotation.x = -Math.PI / 2;
  targetSheet.position.y = y(target);
  const targetEdge = new THREE.LineSegments(
    new THREE.EdgesGeometry(targetSheet.geometry),
    new THREE.LineBasicMaterial({ color: 0x5eead4, transparent: true, opacity: 0.6 }),
  );
  targetEdge.rotation.x = -Math.PI / 2;
  targetEdge.position.y = targetSheet.position.y;
  group.add(targetSheet, targetEdge);

  const curvePoints = points.map((v, i) => new THREE.Vector3(x(i), y(v), 0));
  const curve = new THREE.CatmullRomCurve3(curvePoints.length > 1 ? curvePoints : [curvePoints[0], curvePoints[0].clone().setX(0.01)]);
  const tube = new THREE.Mesh(
    new THREE.TubeGeometry(curve, 160, 0.07, 12, false),
    new THREE.MeshBasicMaterial({ color, transparent: true, opacity: evaluation ? 0.95 : 0.35 }),
  );
  tube.name = 'trajectory';
  group.add(tube);

  // Drop lines from each observation onto the target plane show the shortfall directly.
  curvePoints.forEach((p, i) => {
    const below = points[i] < target;
    const marker = new THREE.Sprite(
      new THREE.SpriteMaterial({ map: glow, color: below ? color : 0xe0f2fe, blending: THREE.AdditiveBlending, depthWrite: false, transparent: true }),
    );
    marker.position.copy(p);
    marker.userData = { base: below ? 0.9 : 0.6, phase: i * 0.7 };
    marker.scale.setScalar(marker.userData.base);
    marker.name = 'marker';
    const drop = new THREE.Line(
      new THREE.BufferGeometry().setFromPoints([p, new THREE.Vector3(p.x, y(target), p.z)]),
      new THREE.LineBasicMaterial({ color: below ? color : 0x64748b, transparent: true, opacity: 0.5 }),
    );
    group.add(marker, drop);
  });

  if (!evaluation) return;

  const lowerSheet = new THREE.Mesh(
    new THREE.PlaneGeometry(WIDTH + 3, 5, 24, 10),
    new THREE.MeshBasicMaterial({ color, wireframe: true, transparent: true, opacity: 0.16 }),
  );
  lowerSheet.rotation.x = -Math.PI / 2;
  lowerSheet.position.y = y(evaluation.lower_bound);
  group.add(lowerSheet);

  // The breach horizon: a ring at the head of the trajectory scaled by breach probability.
  const head = curvePoints[curvePoints.length - 1];
  const ring = new THREE.Group();
  ring.position.copy(head);
  ring.name = 'horizon';
  const radius = 0.5 + evaluation.breach_probability * 1.6;
  ring.add(
    new THREE.Mesh(new THREE.TorusGeometry(radius, 0.035, 12, 96), new THREE.MeshBasicMaterial({ color })),
    new THREE.Mesh(
      new THREE.TorusGeometry(radius * 1.35, 0.012, 8, 96),
      new THREE.MeshBasicMaterial({ color, transparent: true, opacity: 0.4 }),
    ),
  );
  const halo = new THREE.Sprite(
    new THREE.SpriteMaterial({ map: glow, color, blending: THREE.AdditiveBlending, transparent: true, opacity: 0.55, depthWrite: false }),
  );
  halo.scale.setScalar(radius * 3.2);
  ring.add(halo);
  group.add(ring);

  // RuVector analogues orbit the horizon; closer analogues orbit tighter.
  evaluation.analogues.forEach((a, i) => {
    const sat = new THREE.Mesh(
      new THREE.SphereGeometry(0.12, 16, 16),
      new THREE.MeshBasicMaterial({ color: a.outcome.includes('breach') ? 0xf43f5e : 0xa5b4fc }),
    );
    sat.userData = { orbit: radius * 1.6 + Math.min(a.distance, 2) * 1.2, speed: 0.4 + i * 0.13, phase: (i / evaluation.analogues.length) * Math.PI * 2 };
    sat.name = 'analogue';
    ring.add(sat);
  });
}

/** 3D evidence trajectory against the commitment target and calibrated lower bound. */
export function EvidenceScene(props: Props) {
  const data = useRef({ props, dirty: true, bornAt: 0 });

  useEffect(() => {
    data.current = { props, dirty: true, bornAt: 0 };
  }, [props.points, props.target, props.evaluation]); // eslint-disable-line react-hooks/exhaustive-deps

  const host = useThreeScene<HTMLDivElement>(({ scene, camera, pointer }) => {
    camera.position.set(0, 4, 15);
    const glow = glowTexture();
    const grid = new THREE.GridHelper(24, 24, 0x334155, 0x1e293b);
    grid.position.y = -HEIGHT / 2 - 0.6;
    (grid.material as THREE.Material).transparent = true;
    (grid.material as THREE.Material).opacity = 0.5;
    scene.add(grid);
    const group = new THREE.Group();
    scene.add(group);

    return {
      frame(elapsed) {
        const d = data.current;
        if (d.dirty) {
          buildData(group, glow, d.props);
          d.dirty = false;
          d.bornAt = elapsed;
        }
        const age = elapsed - d.bornAt;
        group.traverse((obj) => {
          if (obj.name === 'trajectory') {
            const geo = (obj as THREE.Mesh).geometry;
            const grow = Math.min(1, age / 1.4);
            geo.setDrawRange(0, Math.floor((geo.index?.count ?? 0) * (1 - (1 - grow) ** 3)));
          } else if (obj.name === 'marker') {
            obj.scale.setScalar(obj.userData.base * (1 + Math.sin(elapsed * 2 + obj.userData.phase) * 0.08));
          } else if (obj.name === 'horizon') {
            const s = Math.min(1, age / 1.8);
            obj.scale.setScalar(s * (1 + Math.sin(elapsed * 2.4) * 0.05));
            obj.rotation.y = elapsed * 0.6;
            obj.rotation.x = Math.sin(elapsed * 0.4) * 0.3;
          } else if (obj.name === 'analogue') {
            const { orbit, speed, phase } = obj.userData;
            const a = elapsed * speed + phase;
            obj.position.set(Math.cos(a) * orbit, Math.sin(a * 1.3) * 0.4, Math.sin(a) * orbit);
          }
        });
        const angle = Math.sin(elapsed * 0.15) * 0.45 + pointer.x * 0.25;
        const dist = 17 * Math.max(1, 1.5 / camera.aspect); // pull back on narrow viewports
        camera.position.set(Math.sin(angle) * dist, 3.2 + pointer.y * 1.5, Math.cos(angle) * dist);
        camera.lookAt(0, 0.6, 0);
      },
      dispose: () => glow.dispose(),
    };
  });

  return <div ref={host} role="img" aria-label="Three-dimensional evaluation trajectory" className="absolute inset-0" />;
}
