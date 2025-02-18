import { useEffect } from "react";

export const useWindowLoad = (callback: Function) => {
    useEffect(() => {
        const handleLoad = () => {
            if (callback) {
                callback();
            }
    };
  
    window.addEventListener("load", handleLoad);
  
    return () => {
        window.removeEventListener("load", handleLoad);
    };

    }, [callback]);
};