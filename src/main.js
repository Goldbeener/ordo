import { createApp } from 'vue';
import { MotionPlugin } from '@vueuse/motion';

import App from './App.vue';
import './styles/index.less';

createApp(App).use(MotionPlugin).mount('#app');
