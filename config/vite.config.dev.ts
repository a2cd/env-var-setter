import {mergeConfig} from 'vite'
import baseConfig from './vite.config.base'

export default mergeConfig(
    {
        mode: 'dev',
        server: {
            open: false,
            port: 1420,
            fs: {
                strict: true,
            },
        },
        plugins: [],
    },
    baseConfig
)
