import React from 'react';
import PropTypes from "prop-types";
import { makeStyles } from '@material-ui/core/styles';
import Card from '@material-ui/core/Card';
import CardActions from '@material-ui/core/CardActions';
import CardContent from '@material-ui/core/CardContent';
import CardActionArea from "@material-ui/core/CardActionArea";
import Button from '@material-ui/core/Button';
import Typography from '@material-ui/core/Typography';

import GaiaCardList from "./GaiaCardList";

const useStyles = makeStyles({
  card: {
    minWidth: 275,
  },
  bullet: {
    display: 'inline-block',
    margin: '0 2px',
    transform: 'scale(0.8)',
  },
});

export default function GaiaCard(props) {
  const classes = useStyles();
  const bull = <span className={classes.bullet}>•</span>;
  const {
    title
  } = props;

  return (
    <Card className={classes.card}>
      <CardActionArea onClick={() => console.log("testing")}>
        <CardContent>
          <Typography variant="h5" component="h2">
            {title}
          </Typography>
          <GaiaCardList {...props}/>
        </CardContent>
      </CardActionArea>
    </Card>
  );
}

GaiaCard.propTypes = {
    title: PropTypes.string.isRequired,
    dbScan: PropTypes.bool.isRequired,
    accuracy: PropTypes.number,
    correctlyClustered: PropTypes.number,
    incorrectlyClustered: PropTypes.number,
    anomaly: PropTypes.number,
    clusters: PropTypes.number,
}