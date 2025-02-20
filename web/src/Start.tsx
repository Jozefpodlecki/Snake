import { FC } from "react";

interface Props {

}

const Start: FC<Props> = () => {
    
    return <>
        <div className={`z-3 absolute bg-gray top-0 bg-[#000000AA] size-full flex justify-center items-center pointer-events-none`}>
            <div>
                <h1 className="font-[sigmar] text-6xl text-[#CCCCCC]">Press space to start</h1>
            </div>
        </div>
    </>
}

export default Start;