import React from 'react';
import GaiaCard from "../components/GaiaCard";

export default {
    title: 'Gaia Card',
    component: GaiaCard
}

export const ToStorybook = () => <GaiaCard />

ToStorybook.story = {
    name: 'Basic Gaia Card'
}