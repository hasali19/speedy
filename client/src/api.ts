export interface TestResult {
  id: number;
  timestamp: number;
  ping: number;
  download: number;
  upload: number;
}

export interface ResultsListResponse {
  data: TestResult[];
  meta: {
    count: number;
    prev: string;
    next: string;
  };
}

export async function getResultsWithLimit(limit: number) {
  return getResults("/api/results?limit=" + limit);
}

export async function getResults(path: string): Promise<ResultsListResponse> {
  path = path || "/api/results";
  const res = await fetch(path);
  return await res.json();
}

export async function runTest(): Promise<boolean | undefined> {
  const res = await fetch("/api/run_test", { method: "post" });
  if (res.status === 200) {
    return true;
  } else if (res.status === 409) {
    return false;
  }
}
