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
