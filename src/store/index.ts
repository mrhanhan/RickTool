import { configureStore } from "@reduxjs/toolkit";
import environment from "../pages/environment";
import config from "./configSlice";

export default configureStore({
    reducer: {
        config: config
    }
});