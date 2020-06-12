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

function toFixed(num, fixed) {
  var re = new RegExp('^-?\\d+(?:.\\d{0,' + (fixed || -1) + '})?');
  return num.toString().match(re)[0];
}

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
        <ListItemText primary="Accuracy" secondary={accuracy ? toFixed(accuracy, 2) * 100 + '%' : 'N/A'} />
      </ListItem>
      <ListItem>
        <ListItemAvatar>
          <Avatar>
            <DoneIcon />
          </Avatar>
        </ListItemAvatar>
        <ListItemText primary="Correctly Clustered" secondary={correctlyClustered ?? 'N/A'} />
      </ListItem>
      <ListItem>
        <ListItemAvatar>
          <Avatar>
            <ClearIcon />
          </Avatar>
        </ListItemAvatar>
        <ListItemText primary="Incorrectly Clustered" secondary={incorrectlyClustered ?? 'N/A'} />
      </ListItem>
      <ListItem>
        <ListItemAvatar>
          <Avatar>
            <HelpIcon />
          </Avatar>
        </ListItemAvatar>
        <ListItemText primary="Anomalies" secondary={anomaly ?? 'N/A'} />
      </ListItem>
      <ListItem>
        <ListItemAvatar>
          <Avatar>
            <ScatterPlotIcon />
          </Avatar>
        </ListItemAvatar>
        <ListItemText primary="Cluster Sizes" secondary={clusters?.join(", ") ?? 'N/A'} />
      </ListItem>
    </List>
  );
}

GaiaCardList.propTypes = {
  accuracy: PropTypes.number,
  correctlyClustered: PropTypes.number,
  incorrectlyClustered: PropTypes.number,
  anomaly: PropTypes.number,
  clusters: PropTypes.array,
}