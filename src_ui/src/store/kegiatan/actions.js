import axios from "axios"

export async function kegiatanAction() {
  return axios.get("http://127.0.0.1:8080/v1/kegiatan");
}