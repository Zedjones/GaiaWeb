import React from 'react';
import { makeStyles } from '@material-ui/core/styles';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import ListItemText from '@material-ui/core/ListItemText';
import ListItemAvatar from '@material-ui/core/ListItemAvatar';
import Avatar from '@material-ui/core/Avatar';
import PropTypes from "prop-types";
import DoneIcon from '@material-ui/icons/Done';
import AdjustIcon from "@material-ui/icons/Adjust";
import ClearIcon from '@material-ui/icons/Clear';
import ScatterPlotIcon from "@material-ui/icons/ScatterPlot";
import HelpIcon from "@material-ui/icons/Help";

const useStyles = makeStyles((theme) => ({
  root: {
    width: '100%',
    maxWidth: 360,
    backgroundColor: theme.palette.background.paper,
  },
}));

export default function GaiaCardList(props) {
  const classes = useStyles();
  const {
      accuracy,
      correctlyClustered,
      incorrectlyClustered,
      anomaly,
      clusters
  } = props;


  return (
    <List className={classes.root}>
      <ListItem>
        <ListItemAvatar>
          <Avatar>
            <AdjustIcon />
          </Avatar>
        </ListItemAvatar>
        <ListItemText primary="Accuracy" secondary={accuracy + "%"} />
      </ListItem>
      <ListItem>
        <ListItemAvatar>
          <Avatar>
            <DoneIcon />
          </Avatar>
        </ListItemAvatar>
        <ListItemText primary="Correctly Clustered" secondary={correctlyClustered} />
      </ListItem>
      <ListItem>
        <ListItemAvatar>
          <Avatar>
            <ClearIcon />
          </Avatar>
        </ListItemAvatar>
        <ListItemText primary="Incorrectly Clustered" secondary={incorrectlyClustered} />
      </ListItem>
      <ListItem>
        <ListItemAvatar>
          <Avatar>
            <HelpIcon />
          </Avatar>
        </ListItemAvatar>
        <ListItemText primary="Anomaly" secondary={anomaly} />
      </ListItem>
      <ListItem>
        <ListItemAvatar>
          <Avatar>
            <ScatterPlotIcon />
          </Avatar>
        </ListItemAvatar>
        <ListItemText primary="Cluster Sizes" secondary={clusters.join(", ")} />
      </ListItem>
    </List>
  );
}

GaiaCardList.propTypes = {
    accuracy: PropTypes.number.isRequired,
    correctlyClustered: PropTypes.number.isRequired,
    incorrectlyClustered: PropTypes.number.isRequired,
    anomaly: PropTypes.number.isRequired,
    clusters: PropTypes.array.isRequired,
}