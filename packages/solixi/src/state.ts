import { Application, Container, Ticker } from "pixi.js"
import { createRenderer } from "@bearbroidery/constructables"

export const Solixi = createRenderer<SolixiState>()
export type SolixiState = {
  app: Application,
  ticker: Ticker,
  stage: Container,

  invalidate: () => void,
}
