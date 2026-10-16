#!/usr/bin/env python3
"""Render a transparent Gashat OSD with a compact HUD inside its LCD."""

from __future__ import annotations

import argparse
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont


def load_hud_font(size: int) -> ImageFont.ImageFont:
    for path in (
        "/usr/share/fonts/TTF/DejaVuSansMono-Bold.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSansMono-Bold.ttf",
    ):
        try:
            return ImageFont.truetype(path, size)
        except OSError:
            pass
    return ImageFont.load_default()


def draw_speaker(draw: ImageDraw.ImageDraw, x: int, y: int, scale: int, muted: bool) -> None:
    """Draw a tiny speaker glyph with simple vector shapes (no emoji font dependency)."""
    white = (178, 255, 80, 255)
    draw.polygon(
        [(x, y + scale // 3), (x + scale // 4, y + scale // 3),
         (x + scale // 2, y), (x + scale // 2, y + scale),
         (x + scale // 4, y + 2 * scale // 3), (x, y + 2 * scale // 3)],
        fill=white,
    )
    if muted:
        draw.line((x + scale * 0.64, y + scale * 0.25, x + scale, y + scale * 0.75), fill=(255, 90, 140, 255), width=max(2, scale // 10))
        draw.line((x + scale, y + scale * 0.25, x + scale * 0.64, y + scale * 0.75), fill=(255, 90, 140, 255), width=max(2, scale // 10))
    else:
        draw.arc((x + scale * 0.48, y + scale * 0.12, x + scale * 1.05, y + scale * 0.88), -55, 55, fill=white, width=max(2, scale // 9))
        draw.arc((x + scale * 0.42, y - scale * 0.05, x + scale * 1.25, y + scale * 1.05), -55, 55, fill=white, width=max(2, scale // 10))


def draw_sun(draw: ImageDraw.ImageDraw, x: int, y: int, size: int) -> None:
    color = (217, 199, 255, 255)
    center = (x + size // 2, y + size // 2)
    radius = size // 4
    draw.ellipse((center[0] - radius, center[1] - radius, center[0] + radius, center[1] + radius), outline=color, width=max(2, size // 12))
    for i in range(8):
        import math
        angle = math.pi * i / 4
        x1 = center[0] + int(math.cos(angle) * size * 0.34)
        y1 = center[1] + int(math.sin(angle) * size * 0.34)
        x2 = center[0] + int(math.cos(angle) * size * 0.48)
        y2 = center[1] + int(math.sin(angle) * size * 0.48)
        draw.line((x1, y1, x2, y2), fill=color, width=max(2, size // 14))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--asset", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--percent", required=True, type=int, choices=range(101))
    parser.add_argument("--label", required=True, choices=("VOL", "BRI"))
    parser.add_argument("--bar", required=True, help="RGBA color for active meter segments")
    parser.add_argument("--muted", action="store_true")
    args = parser.parse_args()

    # Crop transparent margins so the Gashat itself fills Mako's icon area.
    gashat = Image.open(args.asset).convert("RGBA")
    bbox = gashat.getchannel("A").getbbox()
    if bbox:
        gashat = gashat.crop(bbox)
    canvas = Image.new("RGBA", gashat.size, (0, 0, 0, 0))
    draw = ImageDraw.Draw(canvas)

    # All HUD graphics sit within the clear LCD area in the supplied PNGs.
    hud_left = int(gashat.width * 0.50)
    hud_right = int(gashat.width * 0.92)
    label_y = int(gashat.height * 0.31)
    font = load_hud_font(max(20, int(gashat.height * 0.085)))
    label = f"{args.label}: {args.percent}%"
    text_color = (255, 243, 90, 255) if args.label == "VOL" else (221, 204, 255, 255)
    # Subtle dark LCD backing preserves contrast while keeping the PNG outside
    # the Gashat silhouette fully transparent.
    draw.rounded_rectangle(
        (hud_left - 8, label_y - 5, hud_right + 4, int(gashat.height * 0.64)),
        radius=max(8, int(gashat.height * 0.025)),
        fill=(7, 8, 25, 150),
    )
    draw.text((hud_left, label_y), label, font=font, fill=text_color, stroke_width=1, stroke_fill=(12, 12, 36, 230))
    if args.label == "VOL":
        draw_speaker(draw, int(gashat.width * 0.82), label_y - 1, int(gashat.height * 0.12), args.muted)
    else:
        draw_sun(draw, int(gashat.width * 0.84), label_y, int(gashat.height * 0.12))

    # Segmented meter, modeled on the reference HUD instead of a single block.
    meter_left = int(gashat.width * 0.50)
    meter_right = int(gashat.width * 0.92)
    meter_top = int(gashat.height * 0.49)
    meter_bottom = int(gashat.height * 0.62)
    segments = 16
    gap = max(3, int(gashat.width * 0.008))
    total_width = meter_right - meter_left
    segment_width = (total_width - gap * (segments - 1)) / segments
    active_segments = round(segments * args.percent / 100)
    fill_color = tuple(int(args.bar[i:i + 2], 16) for i in (1, 3, 5)) + (255,)
    inactive_color = (32, 61, 68, 210) if args.label == "VOL" else (55, 43, 79, 210)
    for index in range(segments):
        x1 = round(meter_left + index * (segment_width + gap))
        x2 = round(x1 + segment_width)
        color = fill_color if index < active_segments and not args.muted else inactive_color
        draw.rounded_rectangle((x1, meter_top, x2, meter_bottom), radius=max(3, gap), fill=color)

    canvas.alpha_composite(gashat)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    canvas.save(args.output, format="PNG", optimize=True)


if __name__ == "__main__":
    main()