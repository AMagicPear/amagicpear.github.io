import json

with open("nanoflow.json", "r") as f:
    data = json.load(f)
    particles = data["particles"]
    # 从颜色字符串中提取 RGB 值
    def extract_rgb(color_str):
        rgb = color_str[4:-1].split(", ")
        return [int(c) for c in rgb]
    
    # 只保留必要字段，从原数据中提取
    simplified_particles = [
        {
            "x": p["baseX"],
            "y": p["baseY"],
            "size": p["size"],
            "color": extract_rgb(p["color"]),
        }
        for p in particles
    ]

simplified_nanoflow = {
    "cwidth": data["cwidth"],
    "cheight": data["cheight"],
    "elasticityFactor": data["elasticityFactor"],
    "maxPushForce": data["maxPushForce"],
    "particles": simplified_particles,
}
    
# 写入简化后的 JSON 文件
with open("./src/assets/simplified_nanoflow.json", "w") as f:
    json.dump(simplified_nanoflow, f, indent=2)
