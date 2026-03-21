export interface EciState {
  epoch: Date;
  pos_x: number;
  pos_y: number;
  pos_z: number;
  vel_x: number;
  vel_y: number;
  vel_z: number;
}

export interface Satellite {
  id: string;
  initial_state: EciState;
}
