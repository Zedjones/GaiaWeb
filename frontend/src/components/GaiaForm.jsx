import React, { useState } from "react";
import { TextField, Button } from "@material-ui/core";

export default function GaiaForm() {
  const [title, setTitle] = useState('');
  return (
    <form noValidate autoComplete="off">
      <TextField
        value={title}
        onChange={(event) => setTitle(event.target.value)}
        label="Title"
        variant="outlined"
      />
    </form>
  )
}