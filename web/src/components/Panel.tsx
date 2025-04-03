
import { FC } from "react";
import { Ellipsis, ChevronLeft } from 'lucide-react';
import { ColorPicker, Select, Slider } from 'antd';
import { Github } from "components/Icons";
import { GameOptions } from "lib/types";
import { AggregationColor } from "antd/es/color-picker/color";

const difficulties = [
    {
        value: "easy",
        label: "Easy"
    },
    {
        value: "hard",
        label: "Hard"
    }
];

const fpsOptions = [
    {
        value: 10,
        label: "10"
    },
    {
        value: 30,
        label: "30"
    },
    {
        value: 60,
        label: "60"
    }
]

interface Props {
    isOpen: boolean;
    options: GameOptions;
    onOptionChange(options: GameOptions): void;
    onToggle(): void;
}

const Panel: FC<Props> = ({
    isOpen,
    options,
    onOptionChange,
    onToggle}) => {

    const onGridSizeChange = (gridSize: number) => {
        onOptionChange({
            ...options,
            gridSize
        })
    }

    const onSnakeColorChange = (value: AggregationColor, css: string) => {
        onOptionChange({
            ...options,
            snakeColor: value.toHex()
        })
    }

    const onDifficultyChange = (difficulty: string) => {
        onOptionChange({
            ...options,
            difficulty: difficulty as "easy" | "hard"
        })
    }

    const onFpsOptionChange = (fps: number) => {
        onOptionChange({
            ...options,
            fps,
            frameThresholdMs: 1000 / fps
        })
    }

    const onFoodCountChange = (foodCount: number) => {
        onOptionChange({
            ...options,
            foodCount
        })
    };

    return <>
        {isOpen ? <div className="z-1 absolute w-full h-full bg-[#000000AA] top-0 left-0"/> : null}
        <div className={`z-2 absolute bg-gray top-0 ${isOpen ? "bg-[#222222AA] w-[300px] h-full": ""}`}>
            {isOpen ?
            <div className="p-2 flex flex-col h-full">
                <div className={`${isOpen ? "": ""} flex justify-end`}>
                    <div className={`${isOpen ? "": ""}`} onClick={onToggle}>
                        <ChevronLeft className="cursor-pointer" />
                    </div>
                </div>
                <div className="grow">
                    <h1 className="text">Settings</h1>
                    <div>
                        <h4 className="my-2">Snake color</h4>
                        <ColorPicker value={options.snakeColor}
                            onChange={onSnakeColorChange} />
                        <h4 className="my-2">Difficulty</h4>
                        <Select
                            defaultValue={options.difficulty}
                            onChange={onDifficultyChange}
                            options={difficulties}
                        />
                        <h4 className="my-2">Fps</h4>
                        <Select
                            defaultValue={options.fps}
                            onChange={onFpsOptionChange}
                            options={fpsOptions}
                        />
                        <h4 className="mt-2">Grid size</h4>
                        <Slider
                            min={20}
                            max={40}
                            value={options.gridSize}
                            onChange={onGridSizeChange}
                            />
                        <h4 className="mt-2">Food count</h4>
                        <Slider
                            min={1}
                            max={20}
                            value={options.foodCount}
                            onChange={onFoodCountChange}
                            />
                        </div>
                </div>
                <div className="flex justify-center items-center">
                    <div className="w-[20px] h-[20px]">
                        <a href="https://github.com/Jozefpodlecki/Snake">
                            <Github/>
                        </a>
                    </div>
                    <div className="ml-4 text-white">
                        Jozef Podlecki 2025
                    </div>
                </div>
            </div> :
            <div className="p-2 flex flex-col h-full" onClick={onToggle}>
                <Ellipsis className="cursor-pointer" />
            </div>}
        </div>
    </>
}

export default Panel;
