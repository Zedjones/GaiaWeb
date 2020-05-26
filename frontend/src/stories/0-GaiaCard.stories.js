import React from 'react';
import GaiaCard from "../components/GaiaCard";

export default {
  title: 'Gaia Card',
  component: GaiaCard
}

export const ToStorybook = () =>
  <GaiaCard 
    title={"Pleiades"} 
    dbScan={true}
    accuracy={86.75}
    correctlyClustered={430}
    incorrectlyClustered={63}
    anomaly={3412}
    clusters={[145, 232, 312]}
  />

ToStorybook.story = {
  name: 'Gaia Card w/ DB Scan'
}