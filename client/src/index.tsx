import React from "react";
import ReactDOM from "react-dom";
import App from "./App";

import { createMuiTheme, ThemeProvider } from "@material-ui/core";
import { orange, blue } from "@material-ui/core/colors";

const theme = createMuiTheme({
  palette: {
    primary: blue,
    secondary: orange,
  },
});

ReactDOM.render(
  <ThemeProvider theme={theme}>
    <App />
  </ThemeProvider>,
  document.getElementById("root")
);
