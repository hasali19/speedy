import React, { useState, useEffect } from "react";
import {
  IconButton,
  AppBar,
  Typography,
  Toolbar,
  Icon,
  Container,
} from "@material-ui/core";
import ResultsTable from "./components/ResultsTable";
import { ResultsListResponse, getResults, getResultsWithLimit } from "./api";

export default function App() {
  const [page, setPage] = useState(0);
  const [rowsPerPage, setRowsPerPage] = useState(3);
  const [results, setResults] = useState<ResultsListResponse | null>(null);

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
      {results && (
        <Container style={{ paddingTop: 24 }}>
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
        </Container>
      )}
    </>
  );
}
