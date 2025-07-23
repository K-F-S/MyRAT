from PIL import Image
import numpy as np
import os

# 画像を読み込み（RGBAモードで読み込み）
img = Image.open("tool/original/Test_Blue_100.jpg").convert("RGBA")

def rgb888_to_rgb565(r, g, b):
    return ((r & 0xF8) << 8) | ((g & 0xFC) << 3) | (b >> 3)

# 出力ディレクトリの作成
os.makedirs("tool/output", exist_ok=True)

with open("tool/output/output.raw", "wb") as f:
    for y in range(img.height):
        for x in range(img.width):
            r, g, b, a = img.getpixel((x, y))
            # 透明な部分は黒（0x0000）に設定
            if a == 0:
                rgb565 = 0x0000
            else:
                rgb565 = rgb888_to_rgb565(r, g, b)
            f.write(rgb565.to_bytes(2, 'big'))  # big endian

print(f"変換完了: tool/output/output.raw (サイズ: {img.width}x{img.height})")