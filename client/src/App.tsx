import React from "react";
import {
  IconButton,
  AppBar,
  Typography,
  Toolbar,
  Icon,
} from "@material-ui/core";

function App() {
  return (
    <>
      <AppBar position="static" color="primary">
        <Toolbar>
          <IconButton edge="start" color="inherit">
            <Icon
              className="mdi mdi-rocket"
              fontSize="large"
              style={{ color: "orange" }}
            />
          </IconButton>
          <Typography variant="h6">Speedy</Typography>
        </Toolbar>
      </AppBar>
    </>
  );
}

export default App;
