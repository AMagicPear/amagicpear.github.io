import coverPyroGenesis from "@/assets/pics/cover_PyroGenesis.webp";
import coverEchoesphere from "@/assets/pics/cover_Echoesphere.jpeg";

export interface PerryWork {
  cardTitle: string;
  classify: string;
  description: string;
  link: string;
  backgroundImg?: string;
  lightColor?: string;
}

export default [
  {
    cardTitle: "燧火启明录",
    classify: "教育游戏",
    description: "一款面向小学高年级学生的中国古代物理探索之旅。",
    link: "https://www.bilibili.com/video/BV16pgBz7EMW",
    backgroundImg: coverPyroGenesis,
    lightColor: "rgba(253, 175, 0, 0.1)",
  },
  {
    cardTitle: "Echoesphere",
    classify: "现场互动游戏",
    description:
      "NJUPT本科毕设作品。探索一种由人工智能驱动的、结合实体的按钮装置的新型游戏交互方式。",
    link: "https://github.com/AMagicPear/Echoesphere",
    backgroundImg: coverEchoesphere,
    lightColor: "rgba(24, 82, 86, 0.1)",
  },
  {
    cardTitle: "PCL.Proto",
    classify: "全栈应用",
    description:
      "Modeled after PCL2 and PCL2-CE, providing a standardized prototype.",
    link: "https://github.com/PCL-Community/PCL.Proto",
    backgroundImg: "https://amagicpear.top/PCL.Proto/PCL.Proto.svg",
    lightColor: "rgba(50, 100, 255, 0.1)",
  },
] satisfies PerryWork[];
