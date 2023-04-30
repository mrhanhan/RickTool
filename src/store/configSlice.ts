import { createSlice } from "@reduxjs/toolkit";
import { updateDataFile } from "../service/store";
import { copy } from "../utils/copy";
import { DataStore } from "./store";
import { VAppGroup } from './vapp';
const data: DataStore = {
    groups: [],
    settings: {}
}

declare type ConfigState = {text: string, data: DataStore};

const configSlice = createSlice({
    name: 'config',
    initialState: {
        data: data,
        text: ''
    },
    reducers: {
        updateConfig: (state) => {
            return state;
        },
        updateText: (state, action) => {
            return { ...state, text: action.payload };
        },
        updateData: (state, action) => {
            return { ...state, data: action.payload };
        },
        addVAppGroup: (state, action) => {
            const newState: ConfigState = copy(state);
            newState.data.groups.push(action.payload);
            updateDataFile(newState.data).then(()=>{});
            console.log(newState, state);
            return newState;
        },
        updateVAppGroup: (state, action) => {
            const newState: ConfigState = copy(state);
            const newGroup = action.payload as VAppGroup;
            newState.data.groups = newState.data.groups.map( group => group.id === newGroup.id ? newGroup: group);
            updateDataFile(newState.data).then(()=>{});
            return newState;
        },
        delVAppGroup: (state, action) => {
            const newState: ConfigState = copy(state);
            const newGroupId = action.payload as string;
            newState.data.groups = newState.data.groups.filter( group => group.id !== newGroupId);
            updateDataFile(newState.data).then(()=>{});
            return newState;
        }
    }
});

export const { updateConfig, updateText, updateData, updateVAppGroup, addVAppGroup, delVAppGroup } = configSlice.actions;

export const selectText = (state: { config: { text: string } }) => state.config.text;
/**
 * 
 * @param state State数据
 * @returns  返回Rick Terminal 的存储数据信息
 */
export const selectData = (state: { config: { data: DataStore } }) => state.config.data;

export default configSlice.reducer;