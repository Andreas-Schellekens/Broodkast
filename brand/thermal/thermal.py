import sys, numpy as np
from PIL import Image
W, H = 1800, 1200
out = sys.argv[1]
def hexc(h): return np.array([int(h[i:i+2], 16) for i in (1, 3, 5)], dtype=np.float32) / 255
stops = [(0.00, "#4A63FF"), (0.16, "#8C3CF2"), (0.32, "#F02D8C"), (0.48, "#FF5A3C"), (0.62, "#FFA23A"),
         (0.74, "#FFD45A"), (0.80, "#E9952F"), (0.90, "#5A2A10"), (1.00, "#0B0705")]
X, Y = np.meshgrid(np.arange(W, dtype=np.float32), np.arange(H, dtype=np.float32))
cx, cy = W * 0.06, H * 0.96
ang = np.deg2rad(33)
u = (X - cx) * np.cos(ang) - (Y - cy) * np.sin(ang)
v = (X - cx) * np.sin(ang) + (Y - cy) * np.cos(ang)
r = np.sqrt((u / (W * 0.88)) ** 2 + (v / (H * 0.64)) ** 2)
r = np.clip(r + 0.05 * np.sin(u / 260.0), 0, 1)
img = np.zeros((H, W, 3), np.float32)
for (p0, c0), (p1, c1) in zip(stops, stops[1:]):
    m = (r >= p0) & (r <= p1)
    t = ((r - p0) / (p1 - p0))[m][:, None]
    img[m] = hexc(c0) * (1 - t) + hexc(c1) * t
rng = np.random.default_rng(3)
img += rng.normal(0, 0.045, (H, W, 1)).astype(np.float32)
img = np.clip(img, 0, 1)
Image.fromarray((img * 255).astype(np.uint8)).save(out, quality=86, method=6)
print(out)
