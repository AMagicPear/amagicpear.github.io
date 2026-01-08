import coverPyroGenesis from "@/assets/pics/cover_PyroGenesis.webp";
import coverEchoesphere from "@/assets/pics/cover_Echoesphere.jpeg";
import type { Language } from "@/i18n";

export interface PerryWork {
  cardTitle: string;
  classify: string;
  description: string;
  link: string;
  backgroundImg?: string;
  lightColor?: string;
}

export type WorksData = Record<Language, PerryWork[]>;

// 共享的基础数据（不随语言变化的部分）
interface WorkBase {
  link: string;
  backgroundImg?: string;
  lightColor?: string;
}

// 本地化字段
interface WorkLocalized {
  cardTitle: string;
  classify: string;
  description: string;
}

// 完整作品定义
interface WorkDefinition extends WorkBase {
  localized: Record<Language, WorkLocalized>;
}

// 作品定义列表
const workDefinitions: WorkDefinition[] = [
  {
    link: "https://www.bilibili.com/video/BV16pgBz7EMW",
    backgroundImg: coverPyroGenesis,
    lightColor: "rgba(253, 175, 0, 0.1)",
    localized: {
      "zh-CN": {
        cardTitle: "燧火启明录",
        classify: "教育游戏",
        description: "一款面向小学高年级学生的中国古代物理探索之旅。"
      },
      "en": {
        cardTitle: "PyroGenesis",
        classify: "Educational Game",
        description: "An exploration journey of ancient Chinese physics for upper elementary school students."
      }
    }
  },
  {
    link: "https://github.com/AMagicPear/Echoesphere",
    backgroundImg: coverEchoesphere,
    lightColor: "rgba(24, 82, 86, 0.1)",
    localized: {
      "zh-CN": {
        cardTitle: "回声之境",
        classify: "展览交互系统",
        description: "NJUPT本科毕设作品。探索一种由人工智能驱动的、结合实体的按钮装置的类游戏化的新型展览交互方式。"
      },
      "en": {
        cardTitle: "Echoesphere",
        classify: "Interactive Installation",
        description: "NJUPT undergraduate graduation project. Exploring a new game interaction method driven by AI and combined with physical button devices."
      }
    }
  },
  {
    link: "https://github.com/PCL-Community/PCL.Proto",
    backgroundImg: "https://amagicpear.top/PCL.Proto/PCL.Proto.svg",
    lightColor: "rgba(50, 100, 255, 0.1)",
    localized: {
      "zh-CN": {
        cardTitle: "PCL.Proto",
        classify: "全栈应用",
        description: "本项目以PCL2（龙腾猫跃）和PCL2-CE为蓝本。旨在为各PCL分支版本提供一个标准化的原型样本。"
      },
      "en": {
        cardTitle: "PCL.Proto",
        classify: "Full-stack Application",
        description: "Modeled after PCL2 and PCL2-CE, providing a standardized prototype."
      }
    }
  }
];

// 生成按语言分组的数据
const worksData: WorksData = {
  "zh-CN": workDefinitions.map(work => ({
    link: work.link,
    backgroundImg: work.backgroundImg,
    lightColor: work.lightColor,
    ...work.localized["zh-CN"]
  })),
  "en": workDefinitions.map(work => ({
    link: work.link,
    backgroundImg: work.backgroundImg,
    lightColor: work.lightColor,
    ...work.localized["en"]
  }))
};

export default worksData;
