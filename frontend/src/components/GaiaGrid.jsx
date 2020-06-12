import React from 'react';
import { Grid } from '@material-ui/core';
import { useQuery, useSubscription } from '@apollo/react-hooks';
import GaiaCard from './GaiaCard';
import PropTypes from "prop-types";
import { gql } from 'apollo-boost';

const GET_COMPUTATIONS = gql`
  query GetComputations($email: String) {
    getComputations(email: $email) {
      id
      email
      title
      correctlyClustered
      incorrectlyClustered
      accuracy
      anomaly
      clusters
      hrPng
      trimmedPng
      distancePng
      pmPng
    }
  }
`;

export default function GaiaGrid(props) {
  const {
    email
  } = props;

  const { loading, data } = useQuery(GET_COMPUTATIONS, {
    variables: { email }
  });

  const mainContainer = () => (
    <Grid container style={{ padding: '2%' }}>
      {data.getComputations.map(computation => (
        <Grid key={computation.id} item>
          <GaiaCard
            {...computation}
          />
        </Grid>
      ))}
    </Grid>
  )
  return (
    <Grid container>
      {loading ? "Loading..." : mainContainer()}
    </Grid>
  )
}

GaiaGrid.propTypes = {
  email: PropTypes.string
}