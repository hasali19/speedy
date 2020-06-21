import React from "react";
import {
  TableContainer,
  Paper,
  Table,
  TableHead,
  TableCell,
  TableRow,
  TableBody,
  TablePagination,
} from "@material-ui/core";

import { TestResult } from "api";

export interface Props {
  results: TestResult[];
  totalCount: number;
  currentPage?: number;
  rowsPerPage?: number;
  onPrevPage?: () => void;
  onNextPage?: () => void;
  onChangeRowsPerPage?: (value: number) => void;
}

function formatUnixTimestamp(timestamp: number) {
  const date = new Date(timestamp * 1000);
  return date;
}

function round2(value: number) {
  return Math.round((value + Number.EPSILON) * 100) / 100;
}

function formatPing(value: number) {
  return round2(value);
}

function formatSpeed(value: number) {
  // Convert from `bytes per second` to `megabits per second`
  const mbps = (value * 8) / 1_000_000;
  return round2(mbps);
}

export default function ResultsTable(props: Props) {
  const {
    results,
    totalCount,
    onPrevPage,
    onNextPage,
    onChangeRowsPerPage,
  } = props;

  const currentPage = props.currentPage || 0;
  const rowsPerPage = props.rowsPerPage || 25;

  function onChangePage(
    e: React.MouseEvent<HTMLButtonElement, MouseEvent> | null,
    page: number
  ) {
    if (page < currentPage && onPrevPage) {
      onPrevPage();
    } else if (page > currentPage && onNextPage) {
      onNextPage();
    }
  }

  return (
    <>
      <TableContainer component={Paper}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>Timestamp</TableCell>
              <TableCell>Ping (ms)</TableCell>
              <TableCell>Download (Mbps)</TableCell>
              <TableCell>Upload (Mbps)</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {results.map(({ id, timestamp, ping, download, upload }) => (
              <TableRow key={id}>
                <TableCell>
                  {formatUnixTimestamp(timestamp).toLocaleString()}
                </TableCell>
                <TableCell>{formatPing(ping)}</TableCell>
                <TableCell>{formatSpeed(download)}</TableCell>
                <TableCell>{formatSpeed(upload)}</TableCell>
              </TableRow>
            ))}
            <TableRow>
              <TablePagination
                count={totalCount}
                page={currentPage}
                onChangePage={onChangePage}
                rowsPerPage={rowsPerPage}
                rowsPerPageOptions={[10, 25, 50, 100]}
                onChangeRowsPerPage={(e) =>
                  onChangeRowsPerPage &&
                  onChangeRowsPerPage(parseInt(e.target.value))
                }
              />
            </TableRow>
          </TableBody>
        </Table>
      </TableContainer>
    </>
  );
}
