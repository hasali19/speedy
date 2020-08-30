import React, { useState, useEffect } from "react";
import {
  IconButton,
  AppBar,
  Typography,
  Toolbar,
  Icon,
  Container,
  Button,
  Snackbar,
  LinearProgress,
} from "@material-ui/core";
import { Alert } from "@material-ui/lab";
import ResultsTable from "./components/ResultsTable";
import {
  ResultsListResponse,
  getResults,
  getResultsWithLimit,
  runTest,
} from "./api";

export default function App() {
  const [page, setPage] = useState(0);
  const [rowsPerPage, setRowsPerPage] = useState(25);
  const [results, setResults] = useState<ResultsListResponse | null>(null);
  const [showAlert, setShowAlert] = useState(false);

  useEffect(() => {
    getResultsWithLimit(rowsPerPage).then(setResults);
  }, [rowsPerPage]);

  async function prevPage() {
    getResults(results!.meta.prev).then((results) => {
      setResults(results);
      setPage(page - 1);
    });
  }

  async function nextPage() {
    getResults(results!.meta.next).then((results) => {
      setResults(results);
      setPage(page + 1);
    });
  }

  async function onRunTestClick() {
    const status = await runTest();
    if (status === false) {
      setShowAlert(true);
    }
  }

  function hideAlert() {
    setShowAlert(false);
  }

  let content;
  if (results) {
    content = (
      <ResultsTable
        results={results.data}
        totalCount={results.meta.count}
        currentPage={page}
        rowsPerPage={rowsPerPage}
        onPrevPage={prevPage}
        onNextPage={nextPage}
        onChangeRowsPerPage={(value) => {
          setRowsPerPage(value);
          setPage(0);
          setResults(null);
        }}
      />
    );
  } else {
    content = <LinearProgress />;
  }

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
          <Typography variant="h6" style={{ flexGrow: 1 }}>
            Speedy
          </Typography>
          <Button variant="outlined" color="inherit" onClick={onRunTestClick}>
            Run test
          </Button>
        </Toolbar>
      </AppBar>
      <Container style={{ paddingTop: 24 }}>{content}</Container>
      <Snackbar open={showAlert} autoHideDuration={2000} onClose={hideAlert}>
        <Alert variant="filled" severity="warning">
          A test is already running.
        </Alert>
      </Snackbar>
    </>
  );
}
