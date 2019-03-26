#![feature(custom_inner_attributes)]
#![rustfmt::skip]
mod common;

macro_rules! norm {
    ($name:ident, $path:expr) => {
        make_spec_test!(Normalization, $name, $path);
    };
}

norm!(spec_normalization_success_haskell_tutorial_access_0, "haskell-tutorial/access/0");
// norm!(spec_normalization_success_haskell_tutorial_access_1, "haskell-tutorial/access/1");
// norm!(spec_normalization_success_haskell_tutorial_combineTypes_0, "haskell-tutorial/combineTypes/0");
// norm!(spec_normalization_success_haskell_tutorial_combineTypes_1, "haskell-tutorial/combineTypes/1");
// norm!(spec_normalization_success_haskell_tutorial_prefer_0, "haskell-tutorial/prefer/0");
norm!(spec_normalization_success_haskell_tutorial_projection_0, "haskell-tutorial/projection/0");


norm!(spec_normalization_success_prelude_Bool_and_0, "prelude/Bool/and/0");
norm!(spec_normalization_success_prelude_Bool_and_1, "prelude/Bool/and/1");
norm!(spec_normalization_success_prelude_Bool_build_0, "prelude/Bool/build/0");
norm!(spec_normalization_success_prelude_Bool_build_1, "prelude/Bool/build/1");
norm!(spec_normalization_success_prelude_Bool_even_0, "prelude/Bool/even/0");
norm!(spec_normalization_success_prelude_Bool_even_1, "prelude/Bool/even/1");
norm!(spec_normalization_success_prelude_Bool_even_2, "prelude/Bool/even/2");
norm!(spec_normalization_success_prelude_Bool_even_3, "prelude/Bool/even/3");
norm!(spec_normalization_success_prelude_Bool_fold_0, "prelude/Bool/fold/0");
norm!(spec_normalization_success_prelude_Bool_fold_1, "prelude/Bool/fold/1");
norm!(spec_normalization_success_prelude_Bool_not_0, "prelude/Bool/not/0");
norm!(spec_normalization_success_prelude_Bool_not_1, "prelude/Bool/not/1");
norm!(spec_normalization_success_prelude_Bool_odd_0, "prelude/Bool/odd/0");
norm!(spec_normalization_success_prelude_Bool_odd_1, "prelude/Bool/odd/1");
norm!(spec_normalization_success_prelude_Bool_odd_2, "prelude/Bool/odd/2");
norm!(spec_normalization_success_prelude_Bool_odd_3, "prelude/Bool/odd/3");
norm!(spec_normalization_success_prelude_Bool_or_0, "prelude/Bool/or/0");
norm!(spec_normalization_success_prelude_Bool_or_1, "prelude/Bool/or/1");
norm!(spec_normalization_success_prelude_Bool_show_0, "prelude/Bool/show/0");
norm!(spec_normalization_success_prelude_Bool_show_1, "prelude/Bool/show/1");
// norm!(spec_normalization_success_prelude_Double_show_0, "prelude/Double/show/0");
// norm!(spec_normalization_success_prelude_Double_show_1, "prelude/Double/show/1");
// norm!(spec_normalization_success_prelude_Integer_show_0, "prelude/Integer/show/0");
// norm!(spec_normalization_success_prelude_Integer_show_1, "prelude/Integer/show/1");
// norm!(spec_normalization_success_prelude_Integer_toDouble_0, "prelude/Integer/toDouble/0");
// norm!(spec_normalization_success_prelude_Integer_toDouble_1, "prelude/Integer/toDouble/1");
norm!(spec_normalization_success_prelude_List_all_0, "prelude/List/all/0");
norm!(spec_normalization_success_prelude_List_all_1, "prelude/List/all/1");
norm!(spec_normalization_success_prelude_List_any_0, "prelude/List/any/0");
norm!(spec_normalization_success_prelude_List_any_1, "prelude/List/any/1");
norm!(spec_normalization_success_prelude_List_build_0, "prelude/List/build/0");
norm!(spec_normalization_success_prelude_List_build_1, "prelude/List/build/1");
norm!(spec_normalization_success_prelude_List_concat_0, "prelude/List/concat/0");
norm!(spec_normalization_success_prelude_List_concat_1, "prelude/List/concat/1");
norm!(spec_normalization_success_prelude_List_concatMap_0, "prelude/List/concatMap/0");
norm!(spec_normalization_success_prelude_List_concatMap_1, "prelude/List/concatMap/1");
norm!(spec_normalization_success_prelude_List_filter_0, "prelude/List/filter/0");
norm!(spec_normalization_success_prelude_List_filter_1, "prelude/List/filter/1");
norm!(spec_normalization_success_prelude_List_fold_0, "prelude/List/fold/0");
norm!(spec_normalization_success_prelude_List_fold_1, "prelude/List/fold/1");
norm!(spec_normalization_success_prelude_List_fold_2, "prelude/List/fold/2");
norm!(spec_normalization_success_prelude_List_generate_0, "prelude/List/generate/0");
norm!(spec_normalization_success_prelude_List_generate_1, "prelude/List/generate/1");
norm!(spec_normalization_success_prelude_List_head_0, "prelude/List/head/0");
norm!(spec_normalization_success_prelude_List_head_1, "prelude/List/head/1");
norm!(spec_normalization_success_prelude_List_indexed_0, "prelude/List/indexed/0");
norm!(spec_normalization_success_prelude_List_indexed_1, "prelude/List/indexed/1");
norm!(spec_normalization_success_prelude_List_iterate_0, "prelude/List/iterate/0");
norm!(spec_normalization_success_prelude_List_iterate_1, "prelude/List/iterate/1");
norm!(spec_normalization_success_prelude_List_last_0, "prelude/List/last/0");
norm!(spec_normalization_success_prelude_List_last_1, "prelude/List/last/1");
norm!(spec_normalization_success_prelude_List_length_0, "prelude/List/length/0");
norm!(spec_normalization_success_prelude_List_length_1, "prelude/List/length/1");
norm!(spec_normalization_success_prelude_List_map_0, "prelude/List/map/0");
norm!(spec_normalization_success_prelude_List_map_1, "prelude/List/map/1");
norm!(spec_normalization_success_prelude_List_null_0, "prelude/List/null/0");
norm!(spec_normalization_success_prelude_List_null_1, "prelude/List/null/1");
norm!(spec_normalization_success_prelude_List_replicate_0, "prelude/List/replicate/0");
norm!(spec_normalization_success_prelude_List_replicate_1, "prelude/List/replicate/1");
norm!(spec_normalization_success_prelude_List_reverse_0, "prelude/List/reverse/0");
norm!(spec_normalization_success_prelude_List_reverse_1, "prelude/List/reverse/1");
norm!(spec_normalization_success_prelude_List_shifted_0, "prelude/List/shifted/0");
norm!(spec_normalization_success_prelude_List_shifted_1, "prelude/List/shifted/1");
norm!(spec_normalization_success_prelude_List_unzip_0, "prelude/List/unzip/0");
norm!(spec_normalization_success_prelude_List_unzip_1, "prelude/List/unzip/1");
norm!(spec_normalization_success_prelude_Natural_build_0, "prelude/Natural/build/0");
norm!(spec_normalization_success_prelude_Natural_build_1, "prelude/Natural/build/1");
norm!(spec_normalization_success_prelude_Natural_enumerate_0, "prelude/Natural/enumerate/0");
norm!(spec_normalization_success_prelude_Natural_enumerate_1, "prelude/Natural/enumerate/1");
norm!(spec_normalization_success_prelude_Natural_even_0, "prelude/Natural/even/0");
norm!(spec_normalization_success_prelude_Natural_even_1, "prelude/Natural/even/1");
norm!(spec_normalization_success_prelude_Natural_fold_0, "prelude/Natural/fold/0");
norm!(spec_normalization_success_prelude_Natural_fold_1, "prelude/Natural/fold/1");
norm!(spec_normalization_success_prelude_Natural_fold_2, "prelude/Natural/fold/2");
norm!(spec_normalization_success_prelude_Natural_isZero_0, "prelude/Natural/isZero/0");
norm!(spec_normalization_success_prelude_Natural_isZero_1, "prelude/Natural/isZero/1");
norm!(spec_normalization_success_prelude_Natural_odd_0, "prelude/Natural/odd/0");
norm!(spec_normalization_success_prelude_Natural_odd_1, "prelude/Natural/odd/1");
norm!(spec_normalization_success_prelude_Natural_product_0, "prelude/Natural/product/0");
norm!(spec_normalization_success_prelude_Natural_product_1, "prelude/Natural/product/1");
norm!(spec_normalization_success_prelude_Natural_show_0, "prelude/Natural/show/0");
norm!(spec_normalization_success_prelude_Natural_show_1, "prelude/Natural/show/1");
norm!(spec_normalization_success_prelude_Natural_sum_0, "prelude/Natural/sum/0");
norm!(spec_normalization_success_prelude_Natural_sum_1, "prelude/Natural/sum/1");
// norm!(spec_normalization_success_prelude_Natural_toDouble_0, "prelude/Natural/toDouble/0");
// norm!(spec_normalization_success_prelude_Natural_toDouble_1, "prelude/Natural/toDouble/1");
norm!(spec_normalization_success_prelude_Natural_toInteger_0, "prelude/Natural/toInteger/0");
norm!(spec_normalization_success_prelude_Natural_toInteger_1, "prelude/Natural/toInteger/1");
norm!(spec_normalization_success_prelude_Optional_all_0, "prelude/Optional/all/0");
norm!(spec_normalization_success_prelude_Optional_all_1, "prelude/Optional/all/1");
norm!(spec_normalization_success_prelude_Optional_any_0, "prelude/Optional/any/0");
norm!(spec_normalization_success_prelude_Optional_any_1, "prelude/Optional/any/1");
norm!(spec_normalization_success_prelude_Optional_build_0, "prelude/Optional/build/0");
norm!(spec_normalization_success_prelude_Optional_build_1, "prelude/Optional/build/1");
norm!(spec_normalization_success_prelude_Optional_concat_0, "prelude/Optional/concat/0");
norm!(spec_normalization_success_prelude_Optional_concat_1, "prelude/Optional/concat/1");
norm!(spec_normalization_success_prelude_Optional_concat_2, "prelude/Optional/concat/2");
norm!(spec_normalization_success_prelude_Optional_filter_0, "prelude/Optional/filter/0");
norm!(spec_normalization_success_prelude_Optional_filter_1, "prelude/Optional/filter/1");
norm!(spec_normalization_success_prelude_Optional_fold_0, "prelude/Optional/fold/0");
norm!(spec_normalization_success_prelude_Optional_fold_1, "prelude/Optional/fold/1");
norm!(spec_normalization_success_prelude_Optional_head_0, "prelude/Optional/head/0");
norm!(spec_normalization_success_prelude_Optional_head_1, "prelude/Optional/head/1");
norm!(spec_normalization_success_prelude_Optional_head_2, "prelude/Optional/head/2");
norm!(spec_normalization_success_prelude_Optional_last_0, "prelude/Optional/last/0");
norm!(spec_normalization_success_prelude_Optional_last_1, "prelude/Optional/last/1");
norm!(spec_normalization_success_prelude_Optional_last_2, "prelude/Optional/last/2");
norm!(spec_normalization_success_prelude_Optional_length_0, "prelude/Optional/length/0");
norm!(spec_normalization_success_prelude_Optional_length_1, "prelude/Optional/length/1");
norm!(spec_normalization_success_prelude_Optional_map_0, "prelude/Optional/map/0");
norm!(spec_normalization_success_prelude_Optional_map_1, "prelude/Optional/map/1");
norm!(spec_normalization_success_prelude_Optional_null_0, "prelude/Optional/null/0");
norm!(spec_normalization_success_prelude_Optional_null_1, "prelude/Optional/null/1");
norm!(spec_normalization_success_prelude_Optional_toList_0, "prelude/Optional/toList/0");
norm!(spec_normalization_success_prelude_Optional_toList_1, "prelude/Optional/toList/1");
norm!(spec_normalization_success_prelude_Optional_unzip_0, "prelude/Optional/unzip/0");
norm!(spec_normalization_success_prelude_Optional_unzip_1, "prelude/Optional/unzip/1");
norm!(spec_normalization_success_prelude_Text_concat_0, "prelude/Text/concat/0");
norm!(spec_normalization_success_prelude_Text_concat_1, "prelude/Text/concat/1");
// norm!(spec_normalization_success_prelude_Text_concatMap_0, "prelude/Text/concatMap/0");
norm!(spec_normalization_success_prelude_Text_concatMap_1, "prelude/Text/concatMap/1");
// norm!(spec_normalization_success_prelude_Text_concatMapSep_0, "prelude/Text/concatMapSep/0");
// norm!(spec_normalization_success_prelude_Text_concatMapSep_1, "prelude/Text/concatMapSep/1");
// norm!(spec_normalization_success_prelude_Text_concatSep_0, "prelude/Text/concatSep/0");
// norm!(spec_normalization_success_prelude_Text_concatSep_1, "prelude/Text/concatSep/1");
// norm!(spec_normalization_success_prelude_Text_show_0, "prelude/Text/show/0");
// norm!(spec_normalization_success_prelude_Text_show_1, "prelude/Text/show/1");



// norm!(spec_normalization_success_remoteSystems, "remoteSystems");
// norm!(spec_normalization_success_simple_doubleShow, "simple/doubleShow");
// norm!(spec_normalization_success_simple_integerShow, "simple/integerShow");
// norm!(spec_normalization_success_simple_integerToDouble, "simple/integerToDouble");
// norm!(spec_normalization_success_simple_letlet, "simple/letlet");
norm!(spec_normalization_success_simple_listBuild, "simple/listBuild");
norm!(spec_normalization_success_simple_multiLine, "simple/multiLine");
norm!(spec_normalization_success_simple_naturalBuild, "simple/naturalBuild");
norm!(spec_normalization_success_simple_naturalPlus, "simple/naturalPlus");
norm!(spec_normalization_success_simple_naturalShow, "simple/naturalShow");
norm!(spec_normalization_success_simple_naturalToInteger, "simple/naturalToInteger");
norm!(spec_normalization_success_simple_optionalBuild, "simple/optionalBuild");
norm!(spec_normalization_success_simple_optionalBuildFold, "simple/optionalBuildFold");
norm!(spec_normalization_success_simple_optionalFold, "simple/optionalFold");
// norm!(spec_normalization_success_simple_sortOperator, "simple/sortOperator");
// norm!(spec_normalization_success_simplifications_and, "simplifications/and");
// norm!(spec_normalization_success_simplifications_eq, "simplifications/eq");
// norm!(spec_normalization_success_simplifications_ifThenElse, "simplifications/ifThenElse");
// norm!(spec_normalization_success_simplifications_ne, "simplifications/ne");
// norm!(spec_normalization_success_simplifications_or, "simplifications/or");


norm!(spec_normalization_success_unit_Bool, "unit/Bool");
norm!(spec_normalization_success_unit_Double, "unit/Double");
norm!(spec_normalization_success_unit_DoubleLiteral, "unit/DoubleLiteral");
norm!(spec_normalization_success_unit_DoubleShow, "unit/DoubleShow");
// norm!(spec_normalization_success_unit_DoubleShowValue, "unit/DoubleShowValue");
norm!(spec_normalization_success_unit_FunctionApplicationCapture, "unit/FunctionApplicationCapture");
norm!(spec_normalization_success_unit_FunctionApplicationNoSubstitute, "unit/FunctionApplicationNoSubstitute");
norm!(spec_normalization_success_unit_FunctionApplicationNormalizeArguments, "unit/FunctionApplicationNormalizeArguments");
norm!(spec_normalization_success_unit_FunctionApplicationSubstitute, "unit/FunctionApplicationSubstitute");
norm!(spec_normalization_success_unit_FunctionNormalizeArguments, "unit/FunctionNormalizeArguments");
norm!(spec_normalization_success_unit_FunctionTypeNormalizeArguments, "unit/FunctionTypeNormalizeArguments");
// norm!(spec_normalization_success_unit_IfAlternativesIdentical, "unit/IfAlternativesIdentical");
norm!(spec_normalization_success_unit_IfFalse, "unit/IfFalse");
norm!(spec_normalization_success_unit_IfNormalizePredicateAndBranches, "unit/IfNormalizePredicateAndBranches");
// norm!(spec_normalization_success_unit_IfTrivial, "unit/IfTrivial");
norm!(spec_normalization_success_unit_IfTrue, "unit/IfTrue");
norm!(spec_normalization_success_unit_Integer, "unit/Integer");
norm!(spec_normalization_success_unit_IntegerNegative, "unit/IntegerNegative");
norm!(spec_normalization_success_unit_IntegerPositive, "unit/IntegerPositive");
// norm!(spec_normalization_success_unit_IntegerShow_12, "unit/IntegerShow-12");
// norm!(spec_normalization_success_unit_IntegerShow12, "unit/IntegerShow12");
norm!(spec_normalization_success_unit_IntegerShow, "unit/IntegerShow");
// norm!(spec_normalization_success_unit_IntegerToDouble_12, "unit/IntegerToDouble-12");
// norm!(spec_normalization_success_unit_IntegerToDouble12, "unit/IntegerToDouble12");
norm!(spec_normalization_success_unit_IntegerToDouble, "unit/IntegerToDouble");
norm!(spec_normalization_success_unit_Kind, "unit/Kind");
norm!(spec_normalization_success_unit_Let, "unit/Let");
norm!(spec_normalization_success_unit_LetWithType, "unit/LetWithType");
norm!(spec_normalization_success_unit_List, "unit/List");
norm!(spec_normalization_success_unit_ListBuild, "unit/ListBuild");
norm!(spec_normalization_success_unit_ListBuildFoldFusion, "unit/ListBuildFoldFusion");
norm!(spec_normalization_success_unit_ListBuildImplementation, "unit/ListBuildImplementation");
norm!(spec_normalization_success_unit_ListFold, "unit/ListFold");
norm!(spec_normalization_success_unit_ListFoldEmpty, "unit/ListFoldEmpty");
norm!(spec_normalization_success_unit_ListFoldOne, "unit/ListFoldOne");
norm!(spec_normalization_success_unit_ListHead, "unit/ListHead");
norm!(spec_normalization_success_unit_ListHeadEmpty, "unit/ListHeadEmpty");
norm!(spec_normalization_success_unit_ListHeadOne, "unit/ListHeadOne");
norm!(spec_normalization_success_unit_ListIndexed, "unit/ListIndexed");
norm!(spec_normalization_success_unit_ListIndexedEmpty, "unit/ListIndexedEmpty");
norm!(spec_normalization_success_unit_ListIndexedOne, "unit/ListIndexedOne");
norm!(spec_normalization_success_unit_ListLast, "unit/ListLast");
norm!(spec_normalization_success_unit_ListLastEmpty, "unit/ListLastEmpty");
norm!(spec_normalization_success_unit_ListLastOne, "unit/ListLastOne");
norm!(spec_normalization_success_unit_ListLength, "unit/ListLength");
norm!(spec_normalization_success_unit_ListLengthEmpty, "unit/ListLengthEmpty");
norm!(spec_normalization_success_unit_ListLengthOne, "unit/ListLengthOne");
norm!(spec_normalization_success_unit_ListNormalizeElements, "unit/ListNormalizeElements");
norm!(spec_normalization_success_unit_ListNormalizeTypeAnnotation, "unit/ListNormalizeTypeAnnotation");
norm!(spec_normalization_success_unit_ListReverse, "unit/ListReverse");
norm!(spec_normalization_success_unit_ListReverseEmpty, "unit/ListReverseEmpty");
norm!(spec_normalization_success_unit_ListReverseTwo, "unit/ListReverseTwo");
// norm!(spec_normalization_success_unit_Merge, "unit/Merge");
norm!(spec_normalization_success_unit_MergeNormalizeArguments, "unit/MergeNormalizeArguments");
norm!(spec_normalization_success_unit_MergeWithType, "unit/MergeWithType");
norm!(spec_normalization_success_unit_MergeWithTypeNormalizeArguments, "unit/MergeWithTypeNormalizeArguments");
norm!(spec_normalization_success_unit_Natural, "unit/Natural");
norm!(spec_normalization_success_unit_NaturalBuild, "unit/NaturalBuild");
norm!(spec_normalization_success_unit_NaturalBuildFoldFusion, "unit/NaturalBuildFoldFusion");
norm!(spec_normalization_success_unit_NaturalBuildImplementation, "unit/NaturalBuildImplementation");
norm!(spec_normalization_success_unit_NaturalEven, "unit/NaturalEven");
norm!(spec_normalization_success_unit_NaturalEvenOne, "unit/NaturalEvenOne");
norm!(spec_normalization_success_unit_NaturalEvenZero, "unit/NaturalEvenZero");
norm!(spec_normalization_success_unit_NaturalFold, "unit/NaturalFold");
norm!(spec_normalization_success_unit_NaturalFoldOne, "unit/NaturalFoldOne");
norm!(spec_normalization_success_unit_NaturalFoldZero, "unit/NaturalFoldZero");
norm!(spec_normalization_success_unit_NaturalIsZero, "unit/NaturalIsZero");
norm!(spec_normalization_success_unit_NaturalIsZeroOne, "unit/NaturalIsZeroOne");
norm!(spec_normalization_success_unit_NaturalIsZeroZero, "unit/NaturalIsZeroZero");
norm!(spec_normalization_success_unit_NaturalLiteral, "unit/NaturalLiteral");
norm!(spec_normalization_success_unit_NaturalOdd, "unit/NaturalOdd");
norm!(spec_normalization_success_unit_NaturalOddOne, "unit/NaturalOddOne");
norm!(spec_normalization_success_unit_NaturalOddZero, "unit/NaturalOddZero");
norm!(spec_normalization_success_unit_NaturalShow, "unit/NaturalShow");
norm!(spec_normalization_success_unit_NaturalShowOne, "unit/NaturalShowOne");
norm!(spec_normalization_success_unit_NaturalToInteger, "unit/NaturalToInteger");
norm!(spec_normalization_success_unit_NaturalToIntegerOne, "unit/NaturalToIntegerOne");
norm!(spec_normalization_success_unit_None, "unit/None");
norm!(spec_normalization_success_unit_NoneNatural, "unit/NoneNatural");
// norm!(spec_normalization_success_unit_OperatorAndEquivalentArguments, "unit/OperatorAndEquivalentArguments");
// norm!(spec_normalization_success_unit_OperatorAndLhsFalse, "unit/OperatorAndLhsFalse");
// norm!(spec_normalization_success_unit_OperatorAndLhsTrue, "unit/OperatorAndLhsTrue");
// norm!(spec_normalization_success_unit_OperatorAndNormalizeArguments, "unit/OperatorAndNormalizeArguments");
// norm!(spec_normalization_success_unit_OperatorAndRhsFalse, "unit/OperatorAndRhsFalse");
// norm!(spec_normalization_success_unit_OperatorAndRhsTrue, "unit/OperatorAndRhsTrue");
// norm!(spec_normalization_success_unit_OperatorEqualEquivalentArguments, "unit/OperatorEqualEquivalentArguments");
// norm!(spec_normalization_success_unit_OperatorEqualLhsTrue, "unit/OperatorEqualLhsTrue");
// norm!(spec_normalization_success_unit_OperatorEqualNormalizeArguments, "unit/OperatorEqualNormalizeArguments");
// norm!(spec_normalization_success_unit_OperatorEqualRhsTrue, "unit/OperatorEqualRhsTrue");
norm!(spec_normalization_success_unit_OperatorListConcatenateLhsEmpty, "unit/OperatorListConcatenateLhsEmpty");
norm!(spec_normalization_success_unit_OperatorListConcatenateListList, "unit/OperatorListConcatenateListList");
norm!(spec_normalization_success_unit_OperatorListConcatenateNormalizeArguments, "unit/OperatorListConcatenateNormalizeArguments");
norm!(spec_normalization_success_unit_OperatorListConcatenateRhsEmpty, "unit/OperatorListConcatenateRhsEmpty");
// norm!(spec_normalization_success_unit_OperatorNotEqualEquivalentArguments, "unit/OperatorNotEqualEquivalentArguments");
// norm!(spec_normalization_success_unit_OperatorNotEqualLhsFalse, "unit/OperatorNotEqualLhsFalse");
// norm!(spec_normalization_success_unit_OperatorNotEqualNormalizeArguments, "unit/OperatorNotEqualNormalizeArguments");
// norm!(spec_normalization_success_unit_OperatorNotEqualRhsFalse, "unit/OperatorNotEqualRhsFalse");
// norm!(spec_normalization_success_unit_OperatorOrEquivalentArguments, "unit/OperatorOrEquivalentArguments");
// norm!(spec_normalization_success_unit_OperatorOrLhsFalse, "unit/OperatorOrLhsFalse");
// norm!(spec_normalization_success_unit_OperatorOrLhsTrue, "unit/OperatorOrLhsTrue");
// norm!(spec_normalization_success_unit_OperatorOrNormalizeArguments, "unit/OperatorOrNormalizeArguments");
// norm!(spec_normalization_success_unit_OperatorOrRhsFalse, "unit/OperatorOrRhsFalse");
// norm!(spec_normalization_success_unit_OperatorOrRhsTrue, "unit/OperatorOrRhsTrue");
// norm!(spec_normalization_success_unit_OperatorPlusLhsZero, "unit/OperatorPlusLhsZero");
// norm!(spec_normalization_success_unit_OperatorPlusNormalizeArguments, "unit/OperatorPlusNormalizeArguments");
norm!(spec_normalization_success_unit_OperatorPlusOneAndOne, "unit/OperatorPlusOneAndOne");
// norm!(spec_normalization_success_unit_OperatorPlusRhsZero, "unit/OperatorPlusRhsZero");
// norm!(spec_normalization_success_unit_OperatorTextConcatenateLhsEmpty, "unit/OperatorTextConcatenateLhsEmpty");
// norm!(spec_normalization_success_unit_OperatorTextConcatenateNormalizeArguments, "unit/OperatorTextConcatenateNormalizeArguments");
// norm!(spec_normalization_success_unit_OperatorTextConcatenateRhsEmpty, "unit/OperatorTextConcatenateRhsEmpty");
norm!(spec_normalization_success_unit_OperatorTextConcatenateTextText, "unit/OperatorTextConcatenateTextText");
// norm!(spec_normalization_success_unit_OperatorTimesLhsOne, "unit/OperatorTimesLhsOne");
// norm!(spec_normalization_success_unit_OperatorTimesLhsZero, "unit/OperatorTimesLhsZero");
// norm!(spec_normalization_success_unit_OperatorTimesNormalizeArguments, "unit/OperatorTimesNormalizeArguments");
// norm!(spec_normalization_success_unit_OperatorTimesRhsOne, "unit/OperatorTimesRhsOne");
// norm!(spec_normalization_success_unit_OperatorTimesRhsZero, "unit/OperatorTimesRhsZero");
norm!(spec_normalization_success_unit_OperatorTimesTwoAndTwo, "unit/OperatorTimesTwoAndTwo");
norm!(spec_normalization_success_unit_Optional, "unit/Optional");
norm!(spec_normalization_success_unit_OptionalBuild, "unit/OptionalBuild");
norm!(spec_normalization_success_unit_OptionalBuildFoldFusion, "unit/OptionalBuildFoldFusion");
norm!(spec_normalization_success_unit_OptionalBuildImplementation, "unit/OptionalBuildImplementation");
norm!(spec_normalization_success_unit_OptionalFold, "unit/OptionalFold");
norm!(spec_normalization_success_unit_OptionalFoldNone, "unit/OptionalFoldNone");
norm!(spec_normalization_success_unit_OptionalFoldSome, "unit/OptionalFoldSome");
norm!(spec_normalization_success_unit_Record, "unit/Record");
norm!(spec_normalization_success_unit_RecordEmpty, "unit/RecordEmpty");
norm!(spec_normalization_success_unit_RecordProjection, "unit/RecordProjection");
norm!(spec_normalization_success_unit_RecordProjectionEmpty, "unit/RecordProjectionEmpty");
norm!(spec_normalization_success_unit_RecordProjectionNormalizeArguments, "unit/RecordProjectionNormalizeArguments");
norm!(spec_normalization_success_unit_RecordSelection, "unit/RecordSelection");
norm!(spec_normalization_success_unit_RecordSelectionNormalizeArguments, "unit/RecordSelectionNormalizeArguments");
norm!(spec_normalization_success_unit_RecordType, "unit/RecordType");
norm!(spec_normalization_success_unit_RecordTypeEmpty, "unit/RecordTypeEmpty");
// norm!(spec_normalization_success_unit_RecursiveRecordMergeCollision, "unit/RecursiveRecordMergeCollision");
// norm!(spec_normalization_success_unit_RecursiveRecordMergeLhsEmpty, "unit/RecursiveRecordMergeLhsEmpty");
// norm!(spec_normalization_success_unit_RecursiveRecordMergeNoCollision, "unit/RecursiveRecordMergeNoCollision");
// norm!(spec_normalization_success_unit_RecursiveRecordMergeNormalizeArguments, "unit/RecursiveRecordMergeNormalizeArguments");
// norm!(spec_normalization_success_unit_RecursiveRecordMergeRhsEmpty, "unit/RecursiveRecordMergeRhsEmpty");
// norm!(spec_normalization_success_unit_RecursiveRecordTypeMergeCollision, "unit/RecursiveRecordTypeMergeCollision");
// norm!(spec_normalization_success_unit_RecursiveRecordTypeMergeLhsEmpty, "unit/RecursiveRecordTypeMergeLhsEmpty");
// norm!(spec_normalization_success_unit_RecursiveRecordTypeMergeNoCollision, "unit/RecursiveRecordTypeMergeNoCollision");
// norm!(spec_normalization_success_unit_RecursiveRecordTypeMergeNormalizeArguments, "unit/RecursiveRecordTypeMergeNormalizeArguments");
// norm!(spec_normalization_success_unit_RecursiveRecordTypeMergeRhsEmpty, "unit/RecursiveRecordTypeMergeRhsEmpty");
// norm!(spec_normalization_success_unit_RightBiasedRecordMergeCollision, "unit/RightBiasedRecordMergeCollision");
// norm!(spec_normalization_success_unit_RightBiasedRecordMergeLhsEmpty, "unit/RightBiasedRecordMergeLhsEmpty");
// norm!(spec_normalization_success_unit_RightBiasedRecordMergeNoCollision, "unit/RightBiasedRecordMergeNoCollision");
// norm!(spec_normalization_success_unit_RightBiasedRecordMergeNormalizeArguments, "unit/RightBiasedRecordMergeNormalizeArguments");
// norm!(spec_normalization_success_unit_RightBiasedRecordMergeRhsEmpty, "unit/RightBiasedRecordMergeRhsEmpty");
norm!(spec_normalization_success_unit_SomeNormalizeArguments, "unit/SomeNormalizeArguments");
norm!(spec_normalization_success_unit_Sort, "unit/Sort");
norm!(spec_normalization_success_unit_Text, "unit/Text");
// norm!(spec_normalization_success_unit_TextInterpolate, "unit/TextInterpolate");
norm!(spec_normalization_success_unit_TextLiteral, "unit/TextLiteral");
norm!(spec_normalization_success_unit_TextNormalizeInterpolations, "unit/TextNormalizeInterpolations");
norm!(spec_normalization_success_unit_TextShow, "unit/TextShow");
// norm!(spec_normalization_success_unit_TextShowAllEscapes, "unit/TextShowAllEscapes");
norm!(spec_normalization_success_unit_True, "unit/True");
norm!(spec_normalization_success_unit_Type, "unit/Type");
norm!(spec_normalization_success_unit_TypeAnnotation, "unit/TypeAnnotation");
// norm!(spec_normalization_success_unit_UnionNormalizeAlternatives, "unit/UnionNormalizeAlternatives");
norm!(spec_normalization_success_unit_UnionNormalizeArguments, "unit/UnionNormalizeArguments");
// norm!(spec_normalization_success_unit_UnionProjectConstructor, "unit/UnionProjectConstructor");
norm!(spec_normalization_success_unit_UnionProjectConstructorNormalizeArguments, "unit/UnionProjectConstructorNormalizeArguments");
// norm!(spec_normalization_success_unit_UnionSortAlternatives, "unit/UnionSortAlternatives");
// norm!(spec_normalization_success_unit_UnionType, "unit/UnionType");
norm!(spec_normalization_success_unit_UnionTypeEmpty, "unit/UnionTypeEmpty");
// norm!(spec_normalization_success_unit_UnionTypeNormalizeArguments, "unit/UnionTypeNormalizeArguments");
norm!(spec_normalization_success_unit_Variable, "unit/Variable");
