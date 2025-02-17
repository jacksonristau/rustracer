import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import Aura from '@primevue/themes/aura'
import PrimeVue from 'primevue/config';
import Button from "primevue/button";
import Image from "primevue/image";
import Card from 'primevue/card';


const app = createApp(App);

app.use(PrimeVue, {
    theme: {
        preset: Aura
    }
});
app.component('button', Button);
app.component('Image', Image);
app.component('Card', Card);

app.mount('#app');
