
import { mchcEnv } from '@lm_fe/env'
import { MyFormSectionForm, MyFormSection } from "@lm_fe/components_m";
import { mchcEvent } from "@lm_fe/env";
import { Form, FormInstance } from "antd";
import React, { useEffect, useState } from "react";
/*[import1]*/
interface IProps {
    form?: FormInstance
}
interface IDataShape {
    name: string
}

/*[lazy]*/


export default (props: IProps) => {
/*[import2]*/
    const [_form] = Form.useForm()
    const form = props.form ?? _form

    useEffect(() => {
        const rm = mchcEvent.on_rm('my_form', e => {
            // mchcEnv.logger.log('event receive', { e })
            if (e.type === 'onChange') {

            }
            else if (e.type === 'onClick') {

            }
        })
        return rm
    }, [])

    //@ts-ignore
    return <MyFormSectionForm<IDataShape> formDescriptions={load_form_config} form={form} onValuesChange={(changedValues, values) => { }} />
    // return <Form form={form}><MyFormSection formDescriptions={load_form_config} form={form} /></Form>

}