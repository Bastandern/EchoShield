import axios from "axios";

const baseURL = import.meta.env.VITE_BASE_URL + "/api/login";
const timeout = 10000;
let headers = {
  "Access-Control-Allow-Origin": "*",
  "Content-Type": "application/json",
};

let request = axios.create({
  baseURL: baseURL,
  timeout: timeout,
  headers: headers,
});

const login = async (requestData) => {
  let res = await request.post("", requestData);
  return res;
};

const register = async (requestData) => {
  let res = await request.put("", requestData);
  return res;
};

export { login, register };
