/*************************************************************************************************

Welcome to Baml! To use this generated code, please run one of the following:

$ npm install @boundaryml/baml
$ yarn add @boundaryml/baml
$ pnpm add @boundaryml/baml

*************************************************************************************************/

// This file was generated by BAML: do not edit it. Instead, edit the BAML
// files and re-generate this code.
//
/* eslint-disable */
// tslint:disable
// @ts-nocheck
// biome-ignore format: autogenerated code
import { FieldType } from '@boundaryml/baml/native'
import { TypeBuilder as _TypeBuilder, EnumBuilder, ClassBuilder } from '@boundaryml/baml/type_builder'

export default class TypeBuilder {
    private tb: _TypeBuilder;
    
    DummyOutput: ClassBuilder<'DummyOutput', "nonce" | "nonce2">;
    
    DynInputOutput: ClassBuilder<'DynInputOutput', "testKey">;
    
    DynamicClassOne: ClassBuilder<'DynamicClassOne'>;
    
    DynamicClassTwo: ClassBuilder<'DynamicClassTwo', "hi" | "some_class" | "status">;
    
    DynamicOutput: ClassBuilder<'DynamicOutput'>;
    
    OriginalB: ClassBuilder<'OriginalB', "value">;
    
    Person: ClassBuilder<'Person', "name" | "hair_color">;
    
    SomeClassNestedDynamic: ClassBuilder<'SomeClassNestedDynamic', "hi">;
    
    
    Color: EnumBuilder<'Color', "RED" | "BLUE" | "GREEN" | "YELLOW" | "BLACK" | "WHITE">;
    
    DynEnumOne: EnumBuilder<'DynEnumOne'>;
    
    DynEnumTwo: EnumBuilder<'DynEnumTwo'>;
    
    Hobby: EnumBuilder<'Hobby', "SPORTS" | "MUSIC" | "READING">;
    

    constructor() {
        this.tb = new _TypeBuilder({
          classes: new Set([
            "BigNumbers","BinaryNode","Blah","BlockConstraint","BlockConstraintForParam","BookOrder","ClassOptionalOutput","ClassOptionalOutput2","ClassWithBlockDone","ClassWithDone","ClassWithImage","ClassWithoutDone","CompoundBigNumbers","ContactInfo","CustomTaskResult","DummyOutput","DynInputOutput","DynamicClassOne","DynamicClassTwo","DynamicOutput","Earthling","Education","Email","EmailAddress","Event","FakeImage","FlightConfirmation","FooAny","Forest","GroceryReceipt","InnerClass","InnerClass2","InputClass","InputClassNested","LinkedList","LiteralClassHello","LiteralClassOne","LiteralClassTwo","MalformedConstraints","MalformedConstraints2","Martian","NamedArgsSingleClass","Nested","Nested2","NestedBlockConstraint","NestedBlockConstraintForParam","Node","OptionalTest_Prop1","OptionalTest_ReturnType","OrderInfo","OriginalA","OriginalB","Person","PhoneNumber","Quantity","RaysData","ReceiptInfo","ReceiptItem","Recipe","Resume","Schema","SearchParams","SomeClassNestedDynamic","StringToClassEntry","TestClassAlias","TestClassNested","TestClassWithEnum","TestOutputClass","Tree","TwoStoriesOneTitle","UnionTest_ReturnType","WithReasoning",
          ]),
          enums: new Set([
            "AliasedEnum","Category","Category2","Category3","Color","DataType","DynEnumOne","DynEnumTwo","EnumInClass","EnumOutput","Hobby","MapKey","NamedArgsSingleEnum","NamedArgsSingleEnumList","OptionalTest_CategoryType","OrderStatus","Tag","TestEnum",
          ])
        });
        
        this.DummyOutput = this.tb.classBuilder("DummyOutput", [
          "nonce","nonce2",
        ]);
        
        this.DynInputOutput = this.tb.classBuilder("DynInputOutput", [
          "testKey",
        ]);
        
        this.DynamicClassOne = this.tb.classBuilder("DynamicClassOne", [
          
        ]);
        
        this.DynamicClassTwo = this.tb.classBuilder("DynamicClassTwo", [
          "hi","some_class","status",
        ]);
        
        this.DynamicOutput = this.tb.classBuilder("DynamicOutput", [
          
        ]);
        
        this.OriginalB = this.tb.classBuilder("OriginalB", [
          "value",
        ]);
        
        this.Person = this.tb.classBuilder("Person", [
          "name","hair_color",
        ]);
        
        this.SomeClassNestedDynamic = this.tb.classBuilder("SomeClassNestedDynamic", [
          "hi",
        ]);
        
        
        this.Color = this.tb.enumBuilder("Color", [
          "RED","BLUE","GREEN","YELLOW","BLACK","WHITE",
        ]);
        
        this.DynEnumOne = this.tb.enumBuilder("DynEnumOne", [
          
        ]);
        
        this.DynEnumTwo = this.tb.enumBuilder("DynEnumTwo", [
          
        ]);
        
        this.Hobby = this.tb.enumBuilder("Hobby", [
          "SPORTS","MUSIC","READING",
        ]);
        
    }

    __tb() {
      return this.tb._tb();
    }
    
    string(): FieldType {
        return this.tb.string()
    }

    literalString(value: string): FieldType {
        return this.tb.literalString(value)
    }

    literalInt(value: number): FieldType {
        return this.tb.literalInt(value)
    }

    literalBool(value: boolean): FieldType {
        return this.tb.literalBool(value)
    }

    int(): FieldType {
        return this.tb.int()
    }

    float(): FieldType {
        return this.tb.float()
    }

    bool(): FieldType {
        return this.tb.bool()
    }

    list(type: FieldType): FieldType {
        return this.tb.list(type)
    }

    null(): FieldType {
        return this.tb.null()
    }

    map(key: FieldType, value: FieldType): FieldType {
        return this.tb.map(key, value)
    }

    union(types: FieldType[]): FieldType {
        return this.tb.union(types)
    }

    addClass<Name extends string>(name: Name): ClassBuilder<Name> {
        return this.tb.addClass(name);
    }

    addEnum<Name extends string>(name: Name): EnumBuilder<Name> {
        return this.tb.addEnum(name);
    }
}