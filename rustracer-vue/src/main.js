import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import Aura from '@primevue/themes/aura'
import PrimeVue from 'primevue/config';
import Button from "primevue/button";
import Image from "primevue/image";
import Card from 'primevue/card';
import InputNumber from 'primevue/inputnumber';
import ColorPicker from 'primevue/colorpicker';

const app = createApp(App);

app.use(PrimeVue, {
    theme: {
        preset: Aura
    }
});
app.component('Button', Button);
app.component('Image', Image);
app.component('Card', Card);
app.component('InputNumber', InputNumber);
app.component('ColorPicker', ColorPicker);

app.mount('#app');
