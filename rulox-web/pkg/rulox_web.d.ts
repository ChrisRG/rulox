/* tslint:disable */
/* eslint-disable */
/**
* @param {string} source
* @returns {WebRulox}
*/
export function run(source: string): WebRulox;
/**
*/
export class WebRulox {
  free(): void;
/**
* @param {string} source
* @returns {WebRulox}
*/
  static new(source: string): WebRulox;
/**
* @returns {any}
*/
  tokens(): any;
/**
* @returns {any}
*/
  parse_tree(): any;
/**
* @returns {any}
*/
  interpret(): any;
/**
* @returns {boolean}
*/
  had_errors(): boolean;
/**
* @returns {string}
*/
  get_environment(): string;
}
