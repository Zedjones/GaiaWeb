import React, { useState } from "react";
import { TextField, Button, Checkbox, FormControl, FormControlLabel } from "@material-ui/core";

export default function GaiaForm() {
  const [title, setTitle] = useState('');
  const [dbScan, setDbScan] = useState(false);

  const ExtraOptions = () => (
    <TextField
      value={title}
      onChange={(event) => setTitle(event.target.value)}
      label="Title"
      variant="standard"
    />
  )

  return (
    <form noValidate autoComplete="off">
      <FormControl>
        <TextField
          value={title}
          onChange={(event) => setTitle(event.target.value)}
          label="Title"
          variant="standard"
        />
        <FormControlLabel
          control={<Checkbox checked={dbScan} onChange={(event) => setDbScan(event.target.checked)} />}
          label="Run DB Scan"
        />
        {
          dbScan ? <ExtraOptions /> : null
        }
      </FormControl>
    </form>
  )
}