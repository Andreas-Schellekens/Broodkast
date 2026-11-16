import random, sys
from PIL import Image, ImageDraw
W, H, SS = 1800, 1100, 2  # supersampled
out = sys.argv[1]
random.seed(7)
img = Image.new("RGBA", (W*SS, H*SS), (0, 0, 0, 0))
d = ImageDraw.Draw(img)
cols = ["#5B5CF0"]*5 + ["#6E6FFF"]*4 + ["#7D86FF"]*3 + ["#8C7BF2"]*3 + ["#4F6BF2"]*3 + ["#A9A6FF"]*2 + ["#7CC8FF"]*1 + ["#3A3FC8"]*3
step = 8
for gy in range(0, H, step):
    for gx in range(0, W, step):
        x = gx + random.uniform(-step*.6, step*.6)
        y = gy + random.uniform(-step*.6, step*.6)
        t = max(0.0, min(1.0, x / W))
        dens = t ** 1.6
        if random.random() > dens * 1.05:
            continue
        r = 0.7 + 3.6 * (t ** 1.4) * random.uniform(.7, 1.15)
        c = random.choice(cols)
        if random.random() < 0.004 * t:
            c = "#FFFFFF"; r = max(r, 2.6)
        a = int(255 * min(1, 0.35 + 0.75 * t))
        rgb = tuple(int(c[i:i+2], 16) for i in (1, 3, 5))
        d.ellipse([(x-r)*SS, (y-r)*SS, (x+r)*SS, (y+r)*SS], fill=rgb + (a,))
img = img.resize((W, H), Image.LANCZOS)
img.save(out, optimize=True)
print(out)
