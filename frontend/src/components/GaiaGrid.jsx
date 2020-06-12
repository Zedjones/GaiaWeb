import React, { useEffect } from 'react';
import { Grid } from '@material-ui/core';
import { useQuery } from '@apollo/react-hooks';
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

const COMPUTATIONS_SUBSCRIPTION = gql`
  subscription ComputationsSubscription($email: String) {
    computations(email: $email){
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

  const { loading, data, subscribeToMore } = useQuery(GET_COMPUTATIONS, {
    variables: { email }
  });

  useEffect(() => {
    if (loading) return;
    return subscribeToMore({
      document: COMPUTATIONS_SUBSCRIPTION,
      variables: { email },
      updateQuery: (prev, { subscriptionData }) => {
        if (!subscriptionData.data) return prev;
        const newComputation = subscriptionData.data.computations;

        if (prev.getComputations.map(val => val.id).includes(newComputation.id)) {
          return {
            getComputations: prev.getComputations.map(val => val.id === newComputation.id ? newComputation : val),
          }
        }
        else {
          return {
            getComputations: [newComputation, ...prev.getComputations],
          };
        }
      }
    })
  }, [loading, subscribeToMore, email])

  const mainContainer = () => (
    <Grid
      container
      style={{ padding: '2%' }}
      spacing={3}
      alignItems="center"
      justify="center"
    >
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
      {loading ? null : mainContainer()}
    </Grid>
  )
}

GaiaGrid.propTypes = {
  email: PropTypes.string
}