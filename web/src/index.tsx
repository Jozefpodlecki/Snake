import { createRoot } from "react-dom/client"
import App from './App'
import "./index.css"
import { ConfigProvider } from "antd";

const container = document.getElementById('root') as HTMLDivElement
const root = createRoot(container);

root.render(
    <ConfigProvider
    theme={{
        components: {
            Slider: {
                handleColor: "#AAAAAA",
                handleActiveColor: "#999999",
                trackBg: "#555555",
                trackHoverBg: "#777777"
            },
        }
    }}
  >
    <App />
  </ConfigProvider>)