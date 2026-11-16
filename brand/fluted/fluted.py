import sys, numpy as np
from PIL import Image
W, H = 1800, 1100
out = sys.argv[1]
def hexc(h): return np.array([int(h[i:i+2], 16) for i in (1, 3, 5)], dtype=np.float32) / 255
def sig(z): return 1 / (1 + np.exp(-z))
def base(x, y):
    xn = x / W
    y1 = H * (0.20 + 0.34 * xn)
    y2 = H * (0.50 + 0.42 * xn)
    a = sig((y - y1) / (H * 0.07))[..., None]
    b = sig((y - y2) / (H * 0.012))[..., None]
    near = np.exp(-((y - y2) / (H * 0.13)) ** 2)[..., None]
    top = hexc("#1A0D07") * (1 - y[..., None] / H * 0.3)
    orange = hexc("#E9792F") * (1 - near) + hexc("#F7B184") * near
    fade = (0.55 + 0.45 * (1 - np.clip(xn - 0.55, 0, 1) * 1.6))[..., None]
    orange = orange * fade
    yn = np.clip((y - y2) / (H * 0.45), 0, 1)[..., None]
    plum = hexc("#4A1838") * (1 - yn) + hexc("#0A1416") * yn
    c = top * (1 - a) + orange * a
    return c * (1 - b) + plum * b
w = 46
X, Y = np.meshgrid(np.arange(W, dtype=np.float32), np.arange(H, dtype=np.float32))
s = (X % w) / w
fs = X - X % w
xs = fs + w / 2 + (s - 0.5) * w * 0.55
ys = Y - H * 0.075 * (s ** 1.6)
img = base(xs, ys)
shade = 0.72 + 0.38 * np.sin(np.pi * s) ** 0.8
img = img * shade[..., None]
edge = np.exp(-((s * w) ** 2) / 1.2)
img = img + edge[..., None] * 0.16
img = np.clip(img, 0, 1)
Image.fromarray((img * 255).astype(np.uint8)).save(out, quality=88, method=6)
print(out)
