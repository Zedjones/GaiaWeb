import React from 'react';
import GaiaForm from "../components/GaiaForm";

export default {
  title: 'Gaia Form',
  component: GaiaForm
}

export const ToStorybook = () =>
  <GaiaForm />

ToStorybook.story = {
  name: 'Gaia Form'
}